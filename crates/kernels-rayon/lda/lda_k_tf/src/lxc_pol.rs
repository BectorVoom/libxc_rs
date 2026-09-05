//! LDA_K_TF lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_tf.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_tf_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_ax: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_ax = f64x8::splat(param_ax);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = t8 * zeta_threshold;
            let t10 = (simd::cbrt(t5));
            let t11 = t10 * t10;
            let t13 = ((t6).select(t9, t11 * t5));
            let t14 = f64x8::splat(1.0) - t4;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(t14));
            let t17 = t16 * t16;
            let t19 = ((t15).select(t9, t17 * t14));
            let t23 = f64x8::splat(M_CBRT3);
            let t24 = param_ax * (t13 / f64x8::splat(2.0) + t19 / f64x8::splat(2.0)) * t23;
            let t26 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t27 = t26 * t26;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = f64x8::splat(M_CBRT4);
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = (simd::cbrt(t2));
            let t33 = t32 * t32;
            let t34 = t31 * t33;
            let t35 = t24 * t34;
            let tzk0 = t35 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t36 = f64x8::splat(5.0) / f64x8::splat(9.0) * t35;
            let t38 = t33 * t2 * param_ax;
            let t39 = t2 * t2;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t1 * t40;
            let t42 = t3 - t41;
            let t45 = ((t6).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t42));
            let t46 = -t42;
            let t49 = ((t15).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t46));
            let t51 = t45 / f64x8::splat(2.0) + t49 / f64x8::splat(2.0);
            let t54 = t23 * t28 * t30;
            let tvrho0 = t36 + t38 * t51 * t54 / f64x8::splat(3.0);
            acc_vrho_0 = tvrho0;
            let t57 = -t3 - t41;
            let t60 = ((t6).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t57));
            let t61 = -t57;
            let t64 = ((t15).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t61));
            let t66 = t60 / f64x8::splat(2.0) + t64 / f64x8::splat(2.0);
            let tvrho1 = t36 + t38 * t66 * t54 / f64x8::splat(3.0);
            acc_vrho_1 = tvrho1;
            let t71 = param_ax * t51 * t23;
            let t72 = t71 * t34;
            let t74 = f64x8::splat(1.0) / t32;
            let t75 = t31 * t74;
            let t77 = f64x8::splat(10.0) / f64x8::splat(27.0) * t24 * t75;
            let t78 = f64x8::splat(1.0) / t10;
            let t79 = t42 * t42;
            let t83 = f64x8::splat(1.0) / t39 / t2;
            let t84 = t1 * t83;
            let t86 = -f64x8::splat(2.0) * t40 + f64x8::splat(2.0) * t84;
            let t90 = ((t6).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t78 * t79 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t86));
            let t91 = f64x8::splat(1.0) / t16;
            let t92 = t46 * t46;
            let t95 = -t86;
            let t99 = ((t15).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t91 * t92 + f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t95));
            let t101 = t90 / f64x8::splat(2.0) + t99 / f64x8::splat(2.0);
            let tv2rho20 = f64x8::splat(10.0) / f64x8::splat(9.0) * t72 + t77 + t38 * t101 * t54 / f64x8::splat(3.0);
            acc_v2rho2_0 = tv2rho20;
            let t106 = t33 * param_ax;
            let t108 = t106 * t66 * t54;
            let t110 = t78 * t57;
            let t113 = t11 * t1;
            let t117 = ((t6).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t110 * t42 + f64x8::splat(10.0) / f64x8::splat(3.0) * t113 * t83));
            let t118 = t91 * t61;
            let t121 = t17 * t1;
            let t125 = ((t15).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t118 * t46 - f64x8::splat(10.0) / f64x8::splat(3.0) * t121 * t83));
            let t127 = t117 / f64x8::splat(2.0) + t125 / f64x8::splat(2.0);
            let tv2rho21 = f64x8::splat(5.0) / f64x8::splat(9.0) * t72 + t77 + f64x8::splat(5.0) / f64x8::splat(9.0) * t108 + t38 * t127 * t54 / f64x8::splat(3.0);
            acc_v2rho2_1 = tv2rho21;
            let t132 = t57 * t57;
            let t136 = f64x8::splat(2.0) * t40 + f64x8::splat(2.0) * t84;
            let t140 = ((t6).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t78 * t132 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t136));
            let t141 = t61 * t61;
            let t144 = -t136;
            let t148 = ((t15).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t91 * t141 + f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t144));
            let t150 = t140 / f64x8::splat(2.0) + t148 / f64x8::splat(2.0);
            let tv2rho22 = f64x8::splat(10.0) / f64x8::splat(9.0) * t108 + t77 + t38 * t150 * t54 / f64x8::splat(3.0);
            acc_v2rho2_2 = tv2rho22;
            let t155 = param_ax * t101 * t23;
            let t156 = t155 * t34;
            let t158 = t71 * t75;
            let t161 = f64x8::splat(1.0) / t32 / t2;
            let t162 = t31 * t161;
            let t164 = f64x8::splat(10.0) / f64x8::splat(81.0) * t24 * t162;
            let t166 = f64x8::splat(1.0) / t10 / t5;
            let t167 = t79 * t42;
            let t170 = t78 * t42;
            let t173 = t39 * t39;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t1 * t174;
            let t177 = f64x8::splat(6.0) * t83 - f64x8::splat(6.0) * t175;
            let t181 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t166 * t167 + f64x8::splat(10.0) / f64x8::splat(3.0) * t170 * t86 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t177));
            let t183 = f64x8::splat(1.0) / t16 / t14;
            let t184 = t92 * t46;
            let t187 = t91 * t46;
            let t190 = -t177;
            let t194 = ((t15).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t183 * t184 + f64x8::splat(10.0) / f64x8::splat(3.0) * t187 * t95 + f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t190));
            let t196 = t181 / f64x8::splat(2.0) + t194 / f64x8::splat(2.0);
            let tv3rho30 = f64x8::splat(5.0) / f64x8::splat(3.0) * t156 + f64x8::splat(10.0) / f64x8::splat(9.0) * t158 - t164 + t38 * t196 * t54 / f64x8::splat(3.0);
            acc_v3rho3_0 = tv3rho30;
            let t202 = t74 * param_ax;
            let t204 = t202 * t66 * t54;
            let t208 = f64x8::splat(10.0) / f64x8::splat(9.0) * t106 * t127 * t54;
            let t209 = t166 * t57;
            let t212 = t78 * t1;
            let t223 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t209 * t79 + f64x8::splat(40.0) / f64x8::splat(9.0) * t212 * t83 * t42 + f64x8::splat(10.0) / f64x8::splat(9.0) * t110 * t86 + f64x8::splat(10.0) / f64x8::splat(3.0) * t11 * t83 - f64x8::splat(10.0) * t113 * t174));
            let t224 = t183 * t61;
            let t227 = t91 * t1;
            let t238 = ((t15).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t224 * t92 - f64x8::splat(40.0) / f64x8::splat(9.0) * t227 * t83 * t46 + f64x8::splat(10.0) / f64x8::splat(9.0) * t118 * t95 - f64x8::splat(10.0) / f64x8::splat(3.0) * t17 * t83 + f64x8::splat(10.0) * t121 * t174));
            let t240 = t223 / f64x8::splat(2.0) + t238 / f64x8::splat(2.0);
            let tv3rho31 = f64x8::splat(5.0) / f64x8::splat(9.0) * t156 + f64x8::splat(20.0) / f64x8::splat(27.0) * t158 - t164 + f64x8::splat(10.0) / f64x8::splat(27.0) * t204 + t208 + t38 * t240 * t54 / f64x8::splat(3.0);
            acc_v3rho3_1 = tv3rho31;
            let t247 = t106 * t150 * t54;
            let t249 = t166 * t132;
            let t254 = t78 * t136;
            let t259 = -f64x8::splat(2.0) * t83 - f64x8::splat(6.0) * t175;
            let t263 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t249 * t42 + f64x8::splat(40.0) / f64x8::splat(9.0) * t110 * t84 + f64x8::splat(10.0) / f64x8::splat(9.0) * t254 * t42 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t259));
            let t264 = t183 * t141;
            let t269 = t91 * t144;
            let t272 = -t259;
            let t276 = ((t15).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t264 * t46 - f64x8::splat(40.0) / f64x8::splat(9.0) * t118 * t84 + f64x8::splat(10.0) / f64x8::splat(9.0) * t269 * t46 + f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t272));
            let t278 = t263 / f64x8::splat(2.0) + t276 / f64x8::splat(2.0);
            let tv3rho32 = f64x8::splat(20.0) / f64x8::splat(27.0) * t204 + t208 + f64x8::splat(10.0) / f64x8::splat(27.0) * t158 - t164 + f64x8::splat(5.0) / f64x8::splat(9.0) * t247 + t38 * t278 * t54 / f64x8::splat(3.0);
            acc_v3rho3_2 = tv3rho32;
            let t284 = t132 * t57;
            let t290 = -f64x8::splat(6.0) * t83 - f64x8::splat(6.0) * t175;
            let t294 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t166 * t284 + f64x8::splat(10.0) / f64x8::splat(3.0) * t110 * t136 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t290));
            let t295 = t141 * t61;
            let t300 = -t290;
            let t304 = ((t15).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t183 * t295 + f64x8::splat(10.0) / f64x8::splat(3.0) * t118 * t144 + f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t300));
            let t306 = t294 / f64x8::splat(2.0) + t304 / f64x8::splat(2.0);
            let tv3rho33 = f64x8::splat(10.0) / f64x8::splat(9.0) * t204 + f64x8::splat(5.0) / f64x8::splat(3.0) * t247 - t164 + t38 * t306 * t54 / f64x8::splat(3.0);
            acc_v3rho3_3 = tv3rho33;
            let t312 = param_ax * t196 * t23 * t34;
            let t314 = t155 * t75;
            let t316 = t71 * t162;
            let t322 = f64x8::splat(40.0) / f64x8::splat(243.0) * t24 * t31 / t32 / t39;
            let t323 = t5 * t5;
            let t325 = f64x8::splat(1.0) / t10 / t323;
            let t326 = t79 * t79;
            let t332 = t86 * t86;
            let t338 = f64x8::splat(1.0) / t173 / t2;
            let t339 = t1 * t338;
            let t341 = -f64x8::splat(24.0) * t174 + f64x8::splat(24.0) * t339;
            let t345 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t325 * t326 - f64x8::splat(20.0) / f64x8::splat(9.0) * t166 * t79 * t86 + f64x8::splat(10.0) / f64x8::splat(3.0) * t78 * t332 + f64x8::splat(40.0) / f64x8::splat(9.0) * t170 * t177 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t341));
            let t346 = t14 * t14;
            let t348 = f64x8::splat(1.0) / t16 / t346;
            let t349 = t92 * t92;
            let t355 = t95 * t95;
            let t364 = ((t15).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t348 * t349 - f64x8::splat(20.0) / f64x8::splat(9.0) * t183 * t92 * t95 + f64x8::splat(10.0) / f64x8::splat(3.0) * t91 * t355 + f64x8::splat(40.0) / f64x8::splat(9.0) * t187 * t190 - f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t341));
            let tv4rho40 = f64x8::splat(20.0) / f64x8::splat(9.0) * t312 + f64x8::splat(20.0) / f64x8::splat(9.0) * t314 - f64x8::splat(40.0) / f64x8::splat(81.0) * t316 + t322 + t38 * (t345 / f64x8::splat(2.0) + t364 / f64x8::splat(2.0)) * t54 / f64x8::splat(3.0);
            acc_v4rho4_0 = tv4rho40;
            let t375 = t161 * param_ax * t66 * t54;
            let t378 = t202 * t127 * t54;
            let t379 = f64x8::splat(10.0) / f64x8::splat(9.0) * t378;
            let t381 = t106 * t240 * t54;
            let t407 = f64x8::splat(40.0) * t113 * t338;
            let t409 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t325 * t57 * t167 - f64x8::splat(20.0) / f64x8::splat(9.0) * t166 * t1 * t83 * t79 - f64x8::splat(10.0) / f64x8::splat(9.0) * t209 * t42 * t86 + f64x8::splat(20.0) / f64x8::splat(3.0) * t78 * t83 * t42 - f64x8::splat(20.0) * t212 * t174 * t42 + f64x8::splat(20.0) / f64x8::splat(3.0) * t212 * t83 * t86 + f64x8::splat(10.0) / f64x8::splat(9.0) * t110 * t177 - f64x8::splat(20.0) * t11 * t174 + t407));
            let t434 = f64x8::splat(40.0) * t121 * t338;
            let t436 = ((t15).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t348 * t61 * t184 + f64x8::splat(20.0) / f64x8::splat(9.0) * t183 * t1 * t83 * t92 - f64x8::splat(10.0) / f64x8::splat(9.0) * t224 * t46 * t95 - f64x8::splat(20.0) / f64x8::splat(3.0) * t91 * t83 * t46 + f64x8::splat(20.0) * t227 * t174 * t46 - f64x8::splat(20.0) / f64x8::splat(3.0) * t227 * t83 * t95 + f64x8::splat(10.0) / f64x8::splat(9.0) * t118 * t190 + f64x8::splat(20.0) * t17 * t174 - t434));
            let tv4rho41 = f64x8::splat(5.0) / f64x8::splat(9.0) * t312 + f64x8::splat(10.0) / f64x8::splat(9.0) * t314 - f64x8::splat(10.0) / f64x8::splat(27.0) * t316 + t322 - f64x8::splat(10.0) / f64x8::splat(81.0) * t375 + t379 + f64x8::splat(5.0) / f64x8::splat(3.0) * t381 + t38 * (t409 / f64x8::splat(2.0) + t436 / f64x8::splat(2.0)) * t54 / f64x8::splat(3.0);
            acc_v4rho4_1 = tv4rho41;
            let t448 = t202 * t150 * t54;
            let t451 = t106 * t278 * t54;
            let t462 = t1 * t1;
            let t465 = f64x8::splat(1.0) / t173 / t39;
            let t481 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t325 * t132 * t79 - f64x8::splat(80.0) / f64x8::splat(27.0) * t209 * t42 * t1 * t83 - f64x8::splat(10.0) / f64x8::splat(27.0) * t249 * t86 + f64x8::splat(80.0) / f64x8::splat(9.0) * t78 * t462 * t465 + f64x8::splat(40.0) / f64x8::splat(9.0) * t110 * t83 - f64x8::splat(40.0) / f64x8::splat(3.0) * t110 * t175 - f64x8::splat(10.0) / f64x8::splat(27.0) * t166 * t136 * t79 + f64x8::splat(20.0) / f64x8::splat(9.0) * t78 * t259 * t42 + f64x8::splat(10.0) / f64x8::splat(9.0) * t254 * t86 + t407));
            let t507 = ((t15).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t348 * t141 * t92 + f64x8::splat(80.0) / f64x8::splat(27.0) * t224 * t46 * t1 * t83 - f64x8::splat(10.0) / f64x8::splat(27.0) * t264 * t95 + f64x8::splat(80.0) / f64x8::splat(9.0) * t91 * t462 * t465 - f64x8::splat(40.0) / f64x8::splat(9.0) * t118 * t83 + f64x8::splat(40.0) / f64x8::splat(3.0) * t118 * t175 - f64x8::splat(10.0) / f64x8::splat(27.0) * t183 * t144 * t92 + f64x8::splat(20.0) / f64x8::splat(9.0) * t91 * t272 * t46 + f64x8::splat(10.0) / f64x8::splat(9.0) * t269 * t95 - t434));
            let tv4rho42 = -f64x8::splat(20.0) / f64x8::splat(81.0) * t375 + f64x8::splat(40.0) / f64x8::splat(27.0) * t378 + f64x8::splat(10.0) / f64x8::splat(9.0) * t381 + f64x8::splat(10.0) / f64x8::splat(27.0) * t314 - f64x8::splat(20.0) / f64x8::splat(81.0) * t316 + t322 + f64x8::splat(10.0) / f64x8::splat(27.0) * t448 + f64x8::splat(10.0) / f64x8::splat(9.0) * t451 + t38 * (t481 / f64x8::splat(2.0) + t507 / f64x8::splat(2.0)) * t54 / f64x8::splat(3.0);
            acc_v4rho4_2 = tv4rho42;
            let t518 = t106 * t306 * t54;
            let t538 = f64x8::splat(12.0) * t174 + f64x8::splat(24.0) * t339;
            let t542 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t325 * t284 * t42 - f64x8::splat(20.0) / f64x8::splat(9.0) * t249 * t84 - f64x8::splat(10.0) / f64x8::splat(9.0) * t209 * t136 * t42 + f64x8::splat(20.0) / f64x8::splat(3.0) * t212 * t83 * t136 + f64x8::splat(10.0) / f64x8::splat(3.0) * t110 * t259 + f64x8::splat(10.0) / f64x8::splat(9.0) * t78 * t290 * t42 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t538));
            let t563 = ((t15).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t348 * t295 * t46 + f64x8::splat(20.0) / f64x8::splat(9.0) * t264 * t84 - f64x8::splat(10.0) / f64x8::splat(9.0) * t224 * t144 * t46 - f64x8::splat(20.0) / f64x8::splat(3.0) * t227 * t83 * t144 + f64x8::splat(10.0) / f64x8::splat(3.0) * t118 * t272 + f64x8::splat(10.0) / f64x8::splat(9.0) * t91 * t300 * t46 - f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t538));
            let tv4rho43 = -f64x8::splat(10.0) / f64x8::splat(27.0) * t375 + t379 + f64x8::splat(10.0) / f64x8::splat(9.0) * t448 + f64x8::splat(5.0) / f64x8::splat(3.0) * t451 - f64x8::splat(10.0) / f64x8::splat(81.0) * t316 + t322 + f64x8::splat(5.0) / f64x8::splat(9.0) * t518 + t38 * (t542 / f64x8::splat(2.0) + t563 / f64x8::splat(2.0)) * t54 / f64x8::splat(3.0);
            acc_v4rho4_3 = tv4rho43;
            let t572 = t132 * t132;
            let t577 = t136 * t136;
            let t583 = f64x8::splat(24.0) * t174 + f64x8::splat(24.0) * t339;
            let t587 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t325 * t572 - f64x8::splat(20.0) / f64x8::splat(9.0) * t249 * t136 + f64x8::splat(10.0) / f64x8::splat(3.0) * t78 * t577 + f64x8::splat(40.0) / f64x8::splat(9.0) * t110 * t290 + f64x8::splat(5.0) / f64x8::splat(3.0) * t11 * t583));
            let t588 = t141 * t141;
            let t593 = t144 * t144;
            let t602 = ((t15).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t348 * t588 - f64x8::splat(20.0) / f64x8::splat(9.0) * t264 * t144 + f64x8::splat(10.0) / f64x8::splat(3.0) * t91 * t593 + f64x8::splat(40.0) / f64x8::splat(9.0) * t118 * t300 - f64x8::splat(5.0) / f64x8::splat(3.0) * t17 * t583));
            let tv4rho44 = -f64x8::splat(40.0) / f64x8::splat(81.0) * t375 + f64x8::splat(20.0) / f64x8::splat(9.0) * t448 + f64x8::splat(20.0) / f64x8::splat(9.0) * t518 + t322 + t38 * (t587 / f64x8::splat(2.0) + t602 / f64x8::splat(2.0)) * t54 / f64x8::splat(3.0);
            acc_v4rho4_4 = tv4rho44;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}
