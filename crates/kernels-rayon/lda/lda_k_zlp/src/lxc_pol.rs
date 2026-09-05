//! LDA_K_ZLP lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
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
pub fn lda_k_zlp_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t2 * t5 * t7;
            let t9 = v_rho0 - v_rho1;
            let t10 = v_rho0 + v_rho1;
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t13 = f64x8::splat(1.0) + t12;
            let t14 = (t13).simd_le(zeta_threshold);
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = ((t14).select(t17, t19 * t13));
            let t22 = f64x8::splat(1.0) - t12;
            let t23 = (t22).simd_le(zeta_threshold);
            let t24 = (simd::cbrt(t22));
            let t25 = t24 * t24;
            let t27 = ((t23).select(t17, t25 * t22));
            let t29 = t21 / f64x8::splat(2.0) + t27 / f64x8::splat(2.0);
            let t30 = (simd::cbrt(t10));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(1.0) / t30;
            let t35 = f64x8::splat(1.0) + f64x8::splat(510.2040816326531) * t33;
            let t36 = (simd::ln(t35));
            let t39 = f64x8::splat(1.0) - f64x8::splat(0.00196) * t30 * t36;
            let t41 = t8 * t32 * t39;
            let tzk0 = f64x8::splat(1.0790666666666666) * t41;
            acc_zk = tzk0;
            let t42 = f64x8::splat(1.7984444444444445) * t41;
            let t43 = t31 * t10;
            let t45 = t43 * t2 * t5;
            let t46 = t10 * t10;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t9 * t47;
            let t49 = t11 - t48;
            let t52 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t49));
            let t53 = -t49;
            let t56 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t53));
            let t58 = t52 / f64x8::splat(2.0) + t56 / f64x8::splat(2.0);
            let t59 = t7 * t58;
            let t63 = t7 * t29;
            let t67 = f64x8::splat(1.0) / t35;
            let t70 = -f64x8::splat(0.0006533333333333333) / t31 * t36 + f64x8::splat(0.3333333333333333) * t11 * t67;
            let t73 = f64x8::splat(1.0790666666666666) * t45 * t63 * t70;
            let tvrho0 = t42 + f64x8::splat(1.0790666666666666) * t45 * t59 * t39 + t73;
            acc_vrho_0 = tvrho0;
            let t74 = -t11 - t48;
            let t77 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t74));
            let t78 = -t74;
            let t81 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t78));
            let t84 = t7 * (t77 / f64x8::splat(2.0) + t81 / f64x8::splat(2.0));
            let t85 = t84 * t39;
            let tvrho1 = t42 + f64x8::splat(1.0790666666666666) * t45 * t85 + t73;
            acc_vrho_1 = tvrho1;
            let t88 = t58 * t31;
            let t90 = t8 * t88 * t39;
            let t92 = t29 * t33;
            let t95 = f64x8::splat(1.198962962962963) * t8 * t92 * t39;
            let t98 = f64x8::splat(3.596888888888889) * t8 * t32 * t70;
            let t99 = f64x8::splat(1.0) / t18;
            let t100 = t49 * t49;
            let t103 = t46 * t10;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t9 * t104;
            let t107 = -f64x8::splat(2.0) * t47 + f64x8::splat(2.0) * t105;
            let t111 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t100 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t107));
            let t112 = f64x8::splat(1.0) / t24;
            let t113 = t53 * t53;
            let t116 = -t107;
            let t120 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t112 * t113 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t116));
            let t122 = t111 / f64x8::splat(2.0) + t120 / f64x8::splat(2.0);
            let t123 = t7 * t122;
            let t128 = t45 * t59 * t70;
            let t136 = f64x8::splat(1.0) / t30 / t46;
            let t137 = t35 * t35;
            let t138 = f64x8::splat(1.0) / t137;
            let t141 = f64x8::splat(0.00043555555555555557) / t43 * t36 - f64x8::splat(0.2222222222222222) * t47 * t67 + f64x8::splat(56.68934240362812) * t136 * t138;
            let t144 = f64x8::splat(1.0790666666666666) * t45 * t63 * t141;
            let tv2rho20 = f64x8::splat(3.596888888888889) * t90 + t95 + t98 + f64x8::splat(1.0790666666666666) * t45 * t123 * t39 + f64x8::splat(2.1581333333333332) * t128 + t144;
            acc_v2rho2_0 = tv2rho20;
            let t147 = t31 * t2 * t5;
            let t148 = t147 * t85;
            let t150 = t99 * t74;
            let t153 = t19 * t9;
            let t157 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t150 * t49 + f64x8::splat(10.0) / f64x8::splat(3.0) * t153 * t104));
            let t158 = t112 * t78;
            let t161 = t25 * t9;
            let t165 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t158 * t53 - f64x8::splat(10.0) / f64x8::splat(3.0) * t161 * t104));
            let t168 = t7 * (t157 / f64x8::splat(2.0) + t165 / f64x8::splat(2.0));
            let t169 = t168 * t39;
            let t172 = t84 * t70;
            let t173 = t45 * t172;
            let tv2rho21 = f64x8::splat(1.7984444444444445) * t90 + t95 + t98 + f64x8::splat(1.7984444444444445) * t148 + f64x8::splat(1.0790666666666666) * t45 * t169 + f64x8::splat(1.0790666666666666) * t173 + f64x8::splat(1.0790666666666666) * t128 + t144;
            acc_v2rho2_1 = tv2rho21;
            let t177 = t74 * t74;
            let t181 = f64x8::splat(2.0) * t47 + f64x8::splat(2.0) * t105;
            let t185 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t177 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t181));
            let t186 = t78 * t78;
            let t189 = -t181;
            let t193 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t112 * t186 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t189));
            let t196 = t7 * (t185 / f64x8::splat(2.0) + t193 / f64x8::splat(2.0));
            let t197 = t196 * t39;
            let tv2rho22 = f64x8::splat(3.596888888888889) * t148 + t95 + t98 + f64x8::splat(1.0790666666666666) * t45 * t197 + f64x8::splat(2.1581333333333332) * t173 + t144;
            acc_v2rho2_2 = tv2rho22;
            let t201 = t122 * t31;
            let t203 = t8 * t201 * t39;
            let t205 = t58 * t33;
            let t207 = t8 * t205 * t39;
            let t210 = t8 * t88 * t70;
            let t213 = f64x8::splat(1.0) / t30 / t10;
            let t214 = t29 * t213;
            let t217 = f64x8::splat(0.3996543209876543) * t8 * t214 * t39;
            let t220 = f64x8::splat(3.596888888888889) * t8 * t92 * t70;
            let t223 = f64x8::splat(5.395333333333333) * t8 * t32 * t141;
            let t225 = f64x8::splat(1.0) / t18 / t13;
            let t226 = t100 * t49;
            let t229 = t99 * t49;
            let t232 = t46 * t46;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t9 * t233;
            let t236 = f64x8::splat(6.0) * t104 - f64x8::splat(6.0) * t234;
            let t240 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t225 * t226 + f64x8::splat(10.0) / f64x8::splat(3.0) * t229 * t107 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t236));
            let t242 = f64x8::splat(1.0) / t24 / t22;
            let t243 = t113 * t53;
            let t246 = t112 * t53;
            let t249 = -t236;
            let t253 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t242 * t243 + f64x8::splat(10.0) / f64x8::splat(3.0) * t246 * t116 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t249));
            let t255 = t240 / f64x8::splat(2.0) + t253 / f64x8::splat(2.0);
            let t256 = t7 * t255;
            let t261 = t45 * t123 * t70;
            let t264 = t45 * t59 * t141;
            let t277 = f64x8::splat(1.0) / t31 / t103;
            let t279 = f64x8::splat(1.0) / t137 / t35;
            let t282 = -f64x8::splat(0.000725925925925926) / t31 / t46 * t36 + f64x8::splat(0.37037037037037035) * t104 * t67 - f64x8::splat(170.06802721088437) / t30 / t103 * t138 + f64x8::splat(19282.089252934733) * t277 * t279;
            let t285 = f64x8::splat(1.0790666666666666) * t45 * t63 * t282;
            let tv3rho30 = f64x8::splat(5.395333333333333) * t203 + f64x8::splat(3.596888888888889) * t207 + f64x8::splat(10.790666666666667) * t210 - t217 + t220 + t223 + f64x8::splat(1.0790666666666666) * t45 * t256 * t39 + f64x8::splat(3.2372) * t261 + f64x8::splat(3.2372) * t264 + t285;
            acc_v3rho3_0 = tv3rho30;
            let t290 = t33 * t2 * t5;
            let t291 = t290 * t85;
            let t294 = f64x8::splat(3.596888888888889) * t147 * t169;
            let t295 = t147 * t172;
            let t297 = t225 * t74;
            let t300 = t99 * t9;
            let t311 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t297 * t100 + f64x8::splat(40.0) / f64x8::splat(9.0) * t300 * t104 * t49 + f64x8::splat(10.0) / f64x8::splat(9.0) * t150 * t107 + f64x8::splat(10.0) / f64x8::splat(3.0) * t19 * t104 - f64x8::splat(10.0) * t153 * t233));
            let t312 = t242 * t78;
            let t315 = t112 * t9;
            let t326 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t312 * t113 - f64x8::splat(40.0) / f64x8::splat(9.0) * t315 * t104 * t53 + f64x8::splat(10.0) / f64x8::splat(9.0) * t158 * t116 - f64x8::splat(10.0) / f64x8::splat(3.0) * t25 * t104 + f64x8::splat(10.0) * t161 * t233));
            let t329 = t7 * (t311 / f64x8::splat(2.0) + t326 / f64x8::splat(2.0));
            let t330 = t329 * t39;
            let t333 = t168 * t70;
            let t335 = f64x8::splat(2.1581333333333332) * t45 * t333;
            let t336 = t84 * t141;
            let t337 = t45 * t336;
            let tv3rho31 = f64x8::splat(1.7984444444444445) * t203 + f64x8::splat(2.397925925925926) * t207 + f64x8::splat(7.193777777777778) * t210 - t217 + t220 + t223 + f64x8::splat(1.198962962962963) * t291 + t294 + f64x8::splat(3.596888888888889) * t295 + f64x8::splat(1.0790666666666666) * t45 * t330 + t335 + f64x8::splat(1.0790666666666666) * t337 + f64x8::splat(1.0790666666666666) * t261 + f64x8::splat(2.1581333333333332) * t264 + t285;
            acc_v3rho3_1 = tv3rho31;
            let t345 = t147 * t197;
            let t347 = t225 * t177;
            let t352 = t99 * t181;
            let t357 = -f64x8::splat(2.0) * t104 - f64x8::splat(6.0) * t234;
            let t361 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t347 * t49 + f64x8::splat(40.0) / f64x8::splat(9.0) * t150 * t105 + f64x8::splat(10.0) / f64x8::splat(9.0) * t352 * t49 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t357));
            let t362 = t242 * t186;
            let t367 = t112 * t189;
            let t370 = -t357;
            let t374 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t362 * t53 - f64x8::splat(40.0) / f64x8::splat(9.0) * t158 * t105 + f64x8::splat(10.0) / f64x8::splat(9.0) * t367 * t53 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t370));
            let t377 = t7 * (t361 / f64x8::splat(2.0) + t374 / f64x8::splat(2.0));
            let t378 = t377 * t39;
            let t381 = t196 * t70;
            let t382 = t45 * t381;
            let tv3rho32 = f64x8::splat(2.397925925925926) * t291 + t294 + f64x8::splat(7.193777777777778) * t295 + f64x8::splat(1.198962962962963) * t207 - t217 + t220 + f64x8::splat(3.596888888888889) * t210 + t223 + f64x8::splat(1.7984444444444445) * t345 + f64x8::splat(1.0790666666666666) * t45 * t378 + f64x8::splat(1.0790666666666666) * t382 + t335 + f64x8::splat(2.1581333333333332) * t337 + f64x8::splat(1.0790666666666666) * t264 + t285;
            acc_v3rho3_2 = tv3rho32;
            let t389 = t177 * t74;
            let t395 = -f64x8::splat(6.0) * t104 - f64x8::splat(6.0) * t234;
            let t399 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t225 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t150 * t181 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t395));
            let t400 = t186 * t78;
            let t405 = -t395;
            let t409 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t242 * t400 + f64x8::splat(10.0) / f64x8::splat(3.0) * t158 * t189 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t405));
            let t412 = t7 * (t399 / f64x8::splat(2.0) + t409 / f64x8::splat(2.0));
            let t413 = t412 * t39;
            let tv3rho33 = f64x8::splat(3.596888888888889) * t291 + f64x8::splat(5.395333333333333) * t345 + f64x8::splat(10.790666666666667) * t295 - t217 + t220 + t223 + f64x8::splat(1.0790666666666666) * t45 * t413 + f64x8::splat(3.2372) * t382 + f64x8::splat(3.2372) * t337 + t285;
            acc_v3rho3_3 = tv3rho33;
            let t420 = t8 * t122 * t33 * t39;
            let t424 = t8 * t58 * t213 * t39;
            let t429 = f64x8::splat(0.5328724279835391) * t8 * t29 * t136 * t39;
            let t432 = t8 * t255 * t31 * t39;
            let t435 = t8 * t201 * t70;
            let t438 = t8 * t205 * t70;
            let t441 = t8 * t88 * t141;
            let t445 = f64x8::splat(1.5986172839506172) * t8 * t214 * t70;
            let t448 = f64x8::splat(7.193777777777778) * t8 * t92 * t141;
            let t451 = f64x8::splat(7.193777777777778) * t8 * t32 * t282;
            let t452 = t13 * t13;
            let t454 = f64x8::splat(1.0) / t18 / t452;
            let t455 = t100 * t100;
            let t461 = t107 * t107;
            let t467 = f64x8::splat(1.0) / t232 / t10;
            let t468 = t9 * t467;
            let t470 = -f64x8::splat(24.0) * t233 + f64x8::splat(24.0) * t468;
            let t474 = ((t14).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t454 * t455 - f64x8::splat(20.0) / f64x8::splat(9.0) * t225 * t100 * t107 + f64x8::splat(10.0) / f64x8::splat(3.0) * t99 * t461 + f64x8::splat(40.0) / f64x8::splat(9.0) * t229 * t236 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t470));
            let t475 = t22 * t22;
            let t477 = f64x8::splat(1.0) / t24 / t475;
            let t478 = t113 * t113;
            let t484 = t116 * t116;
            let t493 = ((t23).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t477 * t478 - f64x8::splat(20.0) / f64x8::splat(9.0) * t242 * t113 * t116 + f64x8::splat(10.0) / f64x8::splat(3.0) * t112 * t484 + f64x8::splat(40.0) / f64x8::splat(9.0) * t246 * t249 - f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t470));
            let t501 = t45 * t256 * t70;
            let t504 = t45 * t123 * t141;
            let t507 = t45 * t59 * t282;
            let t521 = t137 * t137;
            let t528 = f64x8::splat(1.0790666666666666) * t45 * t63 * (f64x8::splat(0.0019358024691358024) * t277 * t36 - f64x8::splat(0.9876543209876543) * t233 * t67 + f64x8::splat(629.8815822625346) / t30 / t232 * t138 - f64x8::splat(128547.26168623156) / t31 / t232 * t279 + f64x8::splat(9837800.639252415) * t467 / t521);
            let tv4rho40 = f64x8::splat(7.193777777777778) * t420 - f64x8::splat(1.5986172839506172) * t424 + t429 + f64x8::splat(7.193777777777778) * t432 + f64x8::splat(21.581333333333333) * t435 + f64x8::splat(14.387555555555556) * t438 + f64x8::splat(21.581333333333333) * t441 - t445 + t448 + t451 + f64x8::splat(1.0790666666666666) * t45 * t7 * (t474 / f64x8::splat(2.0) + t493 / f64x8::splat(2.0)) * t39 + f64x8::splat(4.3162666666666665) * t501 + f64x8::splat(6.4744) * t504 + f64x8::splat(4.3162666666666665) * t507 + t528;
            acc_v4rho4_0 = tv4rho40;
            let t533 = t213 * t2 * t5 * t85;
            let t540 = f64x8::splat(3.596888888888889) * t420 - f64x8::splat(1.198962962962963) * t424 + t429 - f64x8::splat(0.3996543209876543) * t533 + f64x8::splat(1.7984444444444445) * t432 + f64x8::splat(10.790666666666667) * t435 + f64x8::splat(10.790666666666667) * t438 + f64x8::splat(16.186) * t441 - t445 + t448 + t451 + f64x8::splat(1.0790666666666666) * t501;
            let t543 = t290 * t169;
            let t544 = f64x8::splat(3.596888888888889) * t543;
            let t545 = t290 * t172;
            let t547 = t147 * t330;
            let t549 = t147 * t333;
            let t550 = f64x8::splat(10.790666666666667) * t549;
            let t551 = t147 * t336;
            let t577 = f64x8::splat(40.0) * t153 * t467;
            let t579 = ((t14).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t454 * t74 * t226 - f64x8::splat(20.0) / f64x8::splat(9.0) * t225 * t9 * t104 * t100 - f64x8::splat(10.0) / f64x8::splat(9.0) * t297 * t49 * t107 + f64x8::splat(20.0) / f64x8::splat(3.0) * t99 * t104 * t49 - f64x8::splat(20.0) * t300 * t233 * t49 + f64x8::splat(20.0) / f64x8::splat(3.0) * t300 * t104 * t107 + f64x8::splat(10.0) / f64x8::splat(9.0) * t150 * t236 - f64x8::splat(20.0) * t19 * t233 + t577));
            let t604 = f64x8::splat(40.0) * t161 * t467;
            let t606 = ((t23).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t477 * t78 * t243 + f64x8::splat(20.0) / f64x8::splat(9.0) * t242 * t9 * t104 * t113 - f64x8::splat(10.0) / f64x8::splat(9.0) * t312 * t53 * t116 - f64x8::splat(20.0) / f64x8::splat(3.0) * t112 * t104 * t53 + f64x8::splat(20.0) * t315 * t233 * t53 - f64x8::splat(20.0) / f64x8::splat(3.0) * t315 * t104 * t116 + f64x8::splat(10.0) / f64x8::splat(9.0) * t158 * t249 + f64x8::splat(20.0) * t25 * t233 - t604));
            let t614 = t45 * t329 * t70;
            let t617 = t45 * t168 * t141;
            let t618 = f64x8::splat(3.2372) * t617;
            let t620 = t45 * t84 * t282;
            let t622 = f64x8::splat(3.2372) * t504 + f64x8::splat(3.2372) * t507 + t528 + t544 + f64x8::splat(3.596888888888889) * t545 + f64x8::splat(5.395333333333333) * t547 + t550 + f64x8::splat(5.395333333333333) * t551 + f64x8::splat(1.0790666666666666) * t45 * t7 * (t579 / f64x8::splat(2.0) + t606 / f64x8::splat(2.0)) * t39 + f64x8::splat(3.2372) * t614 + t618 + f64x8::splat(1.0790666666666666) * t620;
            let tv4rho41 = t540 + t622;
            acc_v4rho4_1 = tv4rho41;
            let t631 = f64x8::splat(1.198962962962963) * t420 - f64x8::splat(0.7993086419753086) * t424 + t429 + f64x8::splat(3.596888888888889) * t435 + f64x8::splat(7.193777777777778) * t438 + f64x8::splat(10.790666666666667) * t441 - t445 + t448 + t451 + f64x8::splat(1.0790666666666666) * t504 + f64x8::splat(2.1581333333333332) * t507 + t528 - f64x8::splat(0.7993086419753086) * t533;
            let t640 = t290 * t197;
            let t642 = t147 * t378;
            let t644 = t147 * t381;
            let t655 = t9 * t9;
            let t658 = f64x8::splat(1.0) / t232 / t46;
            let t674 = ((t14).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t454 * t177 * t100 - f64x8::splat(80.0) / f64x8::splat(27.0) * t297 * t49 * t9 * t104 - f64x8::splat(10.0) / f64x8::splat(27.0) * t347 * t107 + f64x8::splat(80.0) / f64x8::splat(9.0) * t99 * t655 * t658 + f64x8::splat(40.0) / f64x8::splat(9.0) * t150 * t104 - f64x8::splat(40.0) / f64x8::splat(3.0) * t150 * t234 - f64x8::splat(10.0) / f64x8::splat(27.0) * t225 * t181 * t100 + f64x8::splat(20.0) / f64x8::splat(9.0) * t99 * t357 * t49 + f64x8::splat(10.0) / f64x8::splat(9.0) * t352 * t107 + t577));
            let t700 = ((t23).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t477 * t186 * t113 + f64x8::splat(80.0) / f64x8::splat(27.0) * t312 * t53 * t9 * t104 - f64x8::splat(10.0) / f64x8::splat(27.0) * t362 * t116 + f64x8::splat(80.0) / f64x8::splat(9.0) * t112 * t655 * t658 - f64x8::splat(40.0) / f64x8::splat(9.0) * t158 * t104 + f64x8::splat(40.0) / f64x8::splat(3.0) * t158 * t234 - f64x8::splat(10.0) / f64x8::splat(27.0) * t242 * t189 * t113 + f64x8::splat(20.0) / f64x8::splat(9.0) * t112 * t370 * t53 + f64x8::splat(10.0) / f64x8::splat(9.0) * t367 * t116 - t604));
            let t708 = t45 * t377 * t70;
            let t711 = t45 * t196 * t141;
            let t713 = f64x8::splat(4.795851851851852) * t543 + f64x8::splat(7.193777777777778) * t545 + f64x8::splat(3.596888888888889) * t547 + f64x8::splat(14.387555555555556) * t549 + f64x8::splat(10.790666666666667) * t551 + f64x8::splat(2.1581333333333332) * t614 + f64x8::splat(4.3162666666666665) * t617 + f64x8::splat(2.1581333333333332) * t620 + f64x8::splat(1.198962962962963) * t640 + f64x8::splat(3.596888888888889) * t642 + f64x8::splat(3.596888888888889) * t644 + f64x8::splat(1.0790666666666666) * t45 * t7 * (t674 / f64x8::splat(2.0) + t700 / f64x8::splat(2.0)) * t39 + f64x8::splat(2.1581333333333332) * t708 + f64x8::splat(1.0790666666666666) * t711;
            let tv4rho42 = t631 + t713;
            acc_v4rho4_2 = tv4rho42;
            let t720 = -f64x8::splat(0.3996543209876543) * t424 + t429 - f64x8::splat(1.198962962962963) * t533 + f64x8::splat(3.596888888888889) * t640 + f64x8::splat(3.596888888888889) * t438 + f64x8::splat(5.395333333333333) * t441 - t445 + t448 + t451 + f64x8::splat(1.0790666666666666) * t507 + t528 + t544;
            let t746 = f64x8::splat(12.0) * t233 + f64x8::splat(24.0) * t468;
            let t750 = ((t14).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t454 * t389 * t49 - f64x8::splat(20.0) / f64x8::splat(9.0) * t347 * t105 - f64x8::splat(10.0) / f64x8::splat(9.0) * t297 * t181 * t49 + f64x8::splat(20.0) / f64x8::splat(3.0) * t300 * t104 * t181 + f64x8::splat(10.0) / f64x8::splat(3.0) * t150 * t357 + f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t395 * t49 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t746));
            let t771 = ((t23).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t477 * t400 * t53 + f64x8::splat(20.0) / f64x8::splat(9.0) * t362 * t105 - f64x8::splat(10.0) / f64x8::splat(9.0) * t312 * t189 * t53 - f64x8::splat(20.0) / f64x8::splat(3.0) * t315 * t104 * t189 + f64x8::splat(10.0) / f64x8::splat(3.0) * t158 * t370 + f64x8::splat(10.0) / f64x8::splat(9.0) * t112 * t405 * t53 - f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t746));
            let t779 = t45 * t412 * t70;
            let t781 = t147 * t413;
            let t783 = f64x8::splat(10.790666666666667) * t545 + t550 + f64x8::splat(16.186) * t551 + t618 + f64x8::splat(3.2372) * t620 + f64x8::splat(5.395333333333333) * t642 + f64x8::splat(10.790666666666667) * t644 + f64x8::splat(3.2372) * t708 + f64x8::splat(3.2372) * t711 + f64x8::splat(1.0790666666666666) * t45 * t7 * (t750 / f64x8::splat(2.0) + t771 / f64x8::splat(2.0)) * t39 + f64x8::splat(1.0790666666666666) * t779 + f64x8::splat(1.7984444444444445) * t781;
            let tv4rho43 = t720 + t783;
            acc_v4rho4_3 = tv4rho43;
            let t792 = t177 * t177;
            let t797 = t181 * t181;
            let t803 = f64x8::splat(24.0) * t233 + f64x8::splat(24.0) * t468;
            let t807 = ((t14).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t454 * t792 - f64x8::splat(20.0) / f64x8::splat(9.0) * t347 * t181 + f64x8::splat(10.0) / f64x8::splat(3.0) * t99 * t797 + f64x8::splat(40.0) / f64x8::splat(9.0) * t150 * t395 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t803));
            let t808 = t186 * t186;
            let t813 = t189 * t189;
            let t822 = ((t23).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t477 * t808 - f64x8::splat(20.0) / f64x8::splat(9.0) * t362 * t189 + f64x8::splat(10.0) / f64x8::splat(3.0) * t112 * t813 + f64x8::splat(40.0) / f64x8::splat(9.0) * t158 * t405 - f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t803));
            let tv4rho44 = t429 - f64x8::splat(1.5986172839506172) * t533 + f64x8::splat(7.193777777777778) * t640 - t445 + t448 + t451 + t528 + f64x8::splat(14.387555555555556) * t545 + f64x8::splat(21.581333333333333) * t551 + f64x8::splat(4.3162666666666665) * t620 + f64x8::splat(21.581333333333333) * t644 + f64x8::splat(6.4744) * t711 + f64x8::splat(4.3162666666666665) * t779 + f64x8::splat(1.0790666666666666) * t45 * t7 * (t807 / f64x8::splat(2.0) + t822 / f64x8::splat(2.0)) * t39 + f64x8::splat(7.193777777777778) * t781;
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
