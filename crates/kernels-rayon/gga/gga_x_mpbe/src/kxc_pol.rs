//! GGA_X_MPBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`
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
pub fn gga_x_mpbe_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_c1: f64,
    param_a: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_a = f64x8::splat(param_a);
    let param_c2 = f64x8::splat(param_c2);
    let param_c3 = f64x8::splat(param_c3);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_c1 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = param_a * t28;
            let t42 = t33 * v_sigma0;
            let t46 = f64x8::splat(1.0) + t41 * t42 * t39 / f64x8::splat(24.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t51 = t28 * t28;
            let t52 = param_c2 * t51;
            let t54 = f64x8::splat(1.0) / t31 / t30;
            let t55 = t52 * t54;
            let t56 = v_sigma0 * v_sigma0;
            let t57 = t35 * t35;
            let t58 = t57 * v_rho0;
            let t60 = f64x8::splat(1.0) / t36 / t58;
            let t62 = t46 * t46;
            let t63 = f64x8::splat(1.0) / t62;
            let t67 = t30 * t30;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = param_c3 * t68;
            let t70 = t56 * v_sigma0;
            let t71 = t57 * t57;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t62 * t46;
            let t75 = f64x8::splat(1.0) / t74;
            let t79 = f64x8::splat(1.0) + t34 * v_sigma0 * t39 * t47 / f64x8::splat(24.0) + t55 * t56 * t60 * t63 / f64x8::splat(576.0) + t69 * t70 * t72 * t75 / f64x8::splat(2304.0);
            let t83 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t79));
            let t84 = (v_rho1).simd_le(dens_threshold);
            let t85 = -t16;
            let t87 = ((t14).select(t11, (t10).select(t15, t85 * t7)));
            let t88 = f64x8::splat(1.0) + t87;
            let t89 = (t88).simd_le(zeta_threshold);
            let t90 = (simd::cbrt(t88));
            let t92 = ((t89).select(t22, t90 * t88));
            let t93 = t92 * t26;
            let t94 = v_rho1 * v_rho1;
            let t95 = (simd::cbrt(v_rho1));
            let t96 = t95 * t95;
            let t98 = f64x8::splat(1.0) / t96 / t94;
            let t100 = t33 * v_sigma2;
            let t104 = f64x8::splat(1.0) + t41 * t100 * t98 / f64x8::splat(24.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t109 = v_sigma2 * v_sigma2;
            let t110 = t94 * t94;
            let t111 = t110 * v_rho1;
            let t113 = f64x8::splat(1.0) / t95 / t111;
            let t115 = t104 * t104;
            let t116 = f64x8::splat(1.0) / t115;
            let t120 = t109 * v_sigma2;
            let t121 = t110 * t110;
            let t122 = f64x8::splat(1.0) / t121;
            let t124 = t115 * t104;
            let t125 = f64x8::splat(1.0) / t124;
            let t129 = f64x8::splat(1.0) + t34 * v_sigma2 * t98 * t105 / f64x8::splat(24.0) + t55 * t109 * t113 * t116 / f64x8::splat(576.0) + t69 * t120 * t122 * t125 / f64x8::splat(2304.0);
            let t133 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t129));
            let tzk0 = t83 + t133;
            acc_zk = tzk0;
            let t134 = t6 * t6;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t16 * t135;
            let t138 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t136)));
            let t141 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t138));
            let t142 = t141 * t26;
            let t146 = t26 * t26;
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t25 * t147;
            let t151 = t5 * t148 * t79 / f64x8::splat(8.0);
            let t152 = t35 * v_rho0;
            let t154 = f64x8::splat(1.0) / t37 / t152;
            let t160 = param_c1 * t51 * t54;
            let t161 = t57 * t35;
            let t163 = f64x8::splat(1.0) / t36 / t161;
            let t164 = t56 * t163;
            let t165 = t63 * param_a;
            let t172 = param_c2 * t68;
            let t173 = t172 * t70;
            let t174 = t71 * v_rho0;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t175 * t75;
            let t177 = t176 * param_a;
            let t184 = t56 * t56;
            let t185 = t71 * t152;
            let t187 = f64x8::splat(1.0) / t37 / t185;
            let t190 = t62 * t62;
            let t191 = f64x8::splat(1.0) / t190;
            let t193 = t28 * t33;
            let t194 = t191 * param_a * t193;
            let t197 = -t34 * v_sigma0 * t154 * t47 / f64x8::splat(9.0) + t160 * t164 * t165 / f64x8::splat(216.0) - t55 * t164 * t63 / f64x8::splat(108.0) + t173 * t177 / f64x8::splat(432.0) - t69 * t70 * t175 * t75 / f64x8::splat(288.0) + t69 * t184 * t187 * t194 / f64x8::splat(6912.0);
            let t202 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t79 - t151 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t197));
            let t203 = t85 * t135;
            let t205 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t203)));
            let t208 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t205));
            let t209 = t208 * t26;
            let t213 = t92 * t147;
            let t216 = t5 * t213 * t129 / f64x8::splat(8.0);
            let t218 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t129 - t216));
            let tvrho0 = t83 + t133 + t6 * (t202 + t218);
            acc_vrho_0 = tvrho0;
            let t222 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t136)));
            let t225 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t222));
            let t226 = t225 * t26;
            let t231 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t79 - t151));
            let t233 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t203)));
            let t236 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t233));
            let t237 = t236 * t26;
            let t241 = t94 * v_rho1;
            let t243 = f64x8::splat(1.0) / t96 / t241;
            let t248 = t110 * t94;
            let t250 = f64x8::splat(1.0) / t95 / t248;
            let t251 = t109 * t250;
            let t252 = t116 * param_a;
            let t259 = t172 * t120;
            let t260 = t121 * v_rho1;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t261 * t125;
            let t263 = t262 * param_a;
            let t270 = t109 * t109;
            let t271 = t121 * t241;
            let t273 = f64x8::splat(1.0) / t96 / t271;
            let t276 = t115 * t115;
            let t277 = f64x8::splat(1.0) / t276;
            let t279 = t277 * param_a * t193;
            let t282 = -t34 * v_sigma2 * t243 * t105 / f64x8::splat(9.0) + t160 * t251 * t252 / f64x8::splat(216.0) - t55 * t251 * t116 / f64x8::splat(108.0) + t259 * t263 / f64x8::splat(432.0) - t69 * t120 * t261 * t125 / f64x8::splat(288.0) + t69 * t270 * t273 * t279 / f64x8::splat(6912.0);
            let t287 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t237 * t129 - t216 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t282));
            let tvrho1 = t83 + t133 + t6 * (t231 + t287);
            acc_vrho_1 = tvrho1;
            let t294 = v_sigma0 * t60;
            let t301 = t172 * t56;
            let t302 = t72 * t75;
            let t303 = t302 * param_a;
            let t310 = t71 * t35;
            let t312 = f64x8::splat(1.0) / t37 / t310;
            let t317 = t29 * t33 * t39 * t47 / f64x8::splat(24.0) - t160 * t294 * t165 / f64x8::splat(576.0) + t55 * t294 * t63 / f64x8::splat(288.0) - t301 * t303 / f64x8::splat(1152.0) + t69 * t56 * t72 * t75 / f64x8::splat(768.0) - t69 * t70 * t312 * t194 / f64x8::splat(18432.0);
            let t321 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t317));
            let tvsigma0 = t6 * t321;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t326 = v_sigma2 * t113;
            let t333 = t172 * t109;
            let t334 = t122 * t125;
            let t335 = t334 * param_a;
            let t342 = t121 * t94;
            let t344 = f64x8::splat(1.0) / t96 / t342;
            let t349 = t29 * t33 * t98 * t105 / f64x8::splat(24.0) - t160 * t326 * t252 / f64x8::splat(576.0) + t55 * t326 * t116 / f64x8::splat(288.0) - t333 * t335 / f64x8::splat(1152.0) + t69 * t109 * t122 * t125 / f64x8::splat(768.0) - t69 * t120 * t344 * t279 / f64x8::splat(18432.0);
            let t353 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t349));
            let tvsigma2 = t6 * t353;
            acc_vsigma_2 = tvsigma2;
            let t356 = t23 * t23;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t138 * t138;
            let t361 = t134 * t6;
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = t16 * t362;
            let t366 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t135 + f64x8::splat(2.0) * t363)));
            let t370 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t357 * t358 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t366));
            let t371 = t370 * t26;
            let t375 = t141 * t147;
            let t377 = t5 * t375 * t79;
            let t383 = f64x8::splat(1.0) / t146 / t6;
            let t384 = t25 * t383;
            let t387 = t5 * t384 * t79 / f64x8::splat(12.0);
            let t389 = t5 * t148 * t197;
            let t392 = f64x8::splat(1.0) / t37 / t57;
            let t397 = t57 * t152;
            let t399 = f64x8::splat(1.0) / t36 / t397;
            let t400 = t56 * t399;
            let t404 = param_c1 * t68;
            let t405 = t404 * t70;
            let t406 = f64x8::splat(1.0) / t310;
            let t407 = t406 * t75;
            let t408 = param_a * param_a;
            let t415 = t407 * param_a;
            let t418 = t71 * t57;
            let t420 = f64x8::splat(1.0) / t37 / t418;
            let t421 = t184 * t420;
            let t424 = t191 * t408 * t193;
            let t434 = t184 * v_sigma0;
            let t437 = f64x8::splat(1.0) / t36 / t71 / t397;
            let t441 = f64x8::splat(1.0) / t190 / t46;
            let t443 = t51 * t54;
            let t444 = t441 * t408 * t443;
            let t447 = f64x8::splat(11.0) / f64x8::splat(27.0) * t34 * v_sigma0 * t392 * t47 - t160 * t400 * t165 / f64x8::splat(24.0) + t405 * t407 * t408 / f64x8::splat(162.0) + f64x8::splat(19.0) / f64x8::splat(324.0) * t55 * t400 * t63 - f64x8::splat(43.0) / f64x8::splat(1296.0) * t173 * t415 + t172 * t421 * t424 / f64x8::splat(1296.0) + t69 * t70 * t406 * t75 / f64x8::splat(32.0) - f64x8::splat(59.0) / f64x8::splat(20736.0) * t69 * t421 * t194 + t69 * t434 * t437 * t444 / f64x8::splat(15552.0);
            let t452 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t371 * t79 - t377 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t142 * t197 + t387 - t389 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t447));
            let t453 = t90 * t90;
            let t454 = f64x8::splat(1.0) / t453;
            let t455 = t205 * t205;
            let t458 = t85 * t362;
            let t461 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t135 + f64x8::splat(2.0) * t458)));
            let t465 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t454 * t455 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t461));
            let t466 = t465 * t26;
            let t470 = t208 * t147;
            let t472 = t5 * t470 * t129;
            let t474 = t92 * t383;
            let t477 = t5 * t474 * t129 / f64x8::splat(12.0);
            let t479 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t466 * t129 - t472 / f64x8::splat(4.0) + t477));
            let tv2rho20 = f64x8::splat(2.0) * t202 + f64x8::splat(2.0) * t218 + t6 * (t452 + t479);
            acc_v2rho2_0 = tv2rho20;
            let t482 = t357 * t222;
            let t486 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t363)));
            let t490 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t482 * t138 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t486));
            let t491 = t490 * t26;
            let t495 = t225 * t147;
            let t497 = t5 * t495 * t79;
            let t505 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t491 * t79 - t497 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t197 - t377 / f64x8::splat(8.0) + t387 - t389 / f64x8::splat(8.0)));
            let t506 = t454 * t233;
            let t510 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t458)));
            let t514 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t506 * t205 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t510));
            let t515 = t514 * t26;
            let t519 = t236 * t147;
            let t521 = t5 * t519 * t129;
            let t528 = t5 * t213 * t282;
            let t531 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t515 * t129 - t521 / f64x8::splat(8.0) - t472 / f64x8::splat(8.0) + t477 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t282 - t528 / f64x8::splat(8.0)));
            let tv2rho21 = t202 + t218 + t231 + t287 + t6 * (t505 + t531);
            acc_v2rho2_1 = tv2rho21;
            let t536 = t222 * t222;
            let t541 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t135 + f64x8::splat(2.0) * t363)));
            let t545 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t357 * t536 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t541));
            let t546 = t545 * t26;
            let t552 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t546 * t79 - t497 / f64x8::splat(4.0) + t387));
            let t553 = t233 * t233;
            let t558 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t135 + f64x8::splat(2.0) * t458)));
            let t562 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t454 * t553 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t558));
            let t563 = t562 * t26;
            let t573 = f64x8::splat(1.0) / t96 / t110;
            let t578 = t110 * t241;
            let t580 = f64x8::splat(1.0) / t95 / t578;
            let t581 = t109 * t580;
            let t585 = t404 * t120;
            let t586 = f64x8::splat(1.0) / t342;
            let t587 = t586 * t125;
            let t594 = t587 * param_a;
            let t597 = t121 * t110;
            let t599 = f64x8::splat(1.0) / t96 / t597;
            let t600 = t270 * t599;
            let t603 = t277 * t408 * t193;
            let t613 = t270 * v_sigma2;
            let t616 = f64x8::splat(1.0) / t95 / t121 / t578;
            let t620 = f64x8::splat(1.0) / t276 / t104;
            let t622 = t620 * t408 * t443;
            let t625 = f64x8::splat(11.0) / f64x8::splat(27.0) * t34 * v_sigma2 * t573 * t105 - t160 * t581 * t252 / f64x8::splat(24.0) + t585 * t587 * t408 / f64x8::splat(162.0) + f64x8::splat(19.0) / f64x8::splat(324.0) * t55 * t581 * t116 - f64x8::splat(43.0) / f64x8::splat(1296.0) * t259 * t594 + t172 * t600 * t603 / f64x8::splat(1296.0) + t69 * t120 * t586 * t125 / f64x8::splat(32.0) - f64x8::splat(59.0) / f64x8::splat(20736.0) * t69 * t600 * t279 + t69 * t613 * t616 * t622 / f64x8::splat(15552.0);
            let t630 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t563 * t129 - t521 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t237 * t282 + t477 - t528 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t625));
            let tv2rho22 = f64x8::splat(2.0) * t231 + f64x8::splat(2.0) * t287 + t6 * (t552 + t630);
            acc_v2rho2_2 = tv2rho22;
            let t638 = t5 * t148 * t317 / f64x8::splat(8.0);
            let t643 = t163 * t63;
            let t644 = param_a * v_sigma0;
            let t649 = t176 * t408;
            let t658 = t70 * t187;
            let t669 = t71 * t161;
            let t671 = f64x8::splat(1.0) / t36 / t669;
            let t676 = -t29 * t33 * t154 * t47 / f64x8::splat(9.0) + t160 * t643 * t644 / f64x8::splat(72.0) - t404 * t56 * t649 / f64x8::splat(432.0) - t55 * v_sigma0 * t163 * t63 / f64x8::splat(54.0) + f64x8::splat(5.0) / f64x8::splat(432.0) * t301 * t177 - t172 * t658 * t424 / f64x8::splat(3456.0) - t69 * t56 * t175 * t75 / f64x8::splat(96.0) + f64x8::splat(7.0) / f64x8::splat(6912.0) * t69 * t658 * t194 - t69 * t184 * t671 * t444 / f64x8::splat(41472.0);
            let t681 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t317 - t638 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t676));
            let tv2rhosigma0 = t6 * t681 + t321;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t688 = t5 * t213 * t349 / f64x8::splat(8.0);
            let t690 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t349 - t688));
            let tv2rhosigma2 = t6 * t690 + t353;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t696 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t317 - t638));
            let tv2rhosigma3 = t6 * t696 + t321;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t705 = t250 * t116;
            let t706 = param_a * v_sigma2;
            let t711 = t262 * t408;
            let t720 = t120 * t273;
            let t731 = t121 * t248;
            let t733 = f64x8::splat(1.0) / t95 / t731;
            let t738 = -t29 * t33 * t243 * t105 / f64x8::splat(9.0) + t160 * t705 * t706 / f64x8::splat(72.0) - t404 * t109 * t711 / f64x8::splat(432.0) - t55 * v_sigma2 * t250 * t116 / f64x8::splat(54.0) + f64x8::splat(5.0) / f64x8::splat(432.0) * t333 * t263 - t172 * t720 * t603 / f64x8::splat(3456.0) - t69 * t109 * t261 * t125 / f64x8::splat(96.0) + f64x8::splat(7.0) / f64x8::splat(6912.0) * t69 * t720 * t279 - t69 * t270 * t733 * t622 / f64x8::splat(41472.0);
            let t743 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t237 * t349 - t688 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t738));
            let tv2rhosigma5 = t6 * t743 + t353;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t750 = t302 * t408;
            let t760 = t56 * t312;
            let t771 = t71 * t58;
            let t773 = f64x8::splat(1.0) / t36 / t771;
            let t778 = -t160 * t60 * t63 * param_a / f64x8::splat(288.0) + t404 * v_sigma0 * t750 / f64x8::splat(1152.0) + t52 * t54 * t60 * t63 / f64x8::splat(288.0) - t172 * v_sigma0 * t303 / f64x8::splat(288.0) + t172 * t760 * t424 / f64x8::splat(9216.0) + t69 * v_sigma0 * t72 * t75 / f64x8::splat(384.0) - t69 * t760 * t194 / f64x8::splat(3072.0) + t69 * t70 * t773 * t444 / f64x8::splat(110592.0);
            let t782 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t778));
            let tv2sigma20 = t6 * t782;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t788 = t334 * t408;
            let t798 = t109 * t344;
            let t809 = t121 * t111;
            let t811 = f64x8::splat(1.0) / t95 / t809;
            let t816 = -t160 * t113 * t116 * param_a / f64x8::splat(288.0) + t404 * v_sigma2 * t788 / f64x8::splat(1152.0) + t52 * t54 * t113 * t116 / f64x8::splat(288.0) - t172 * v_sigma2 * t335 / f64x8::splat(288.0) + t172 * t798 * t603 / f64x8::splat(9216.0) + t69 * v_sigma2 * t122 * t125 / f64x8::splat(384.0) - t69 * t798 * t279 / f64x8::splat(3072.0) + t69 * t120 * t811 * t622 / f64x8::splat(110592.0);
            let t820 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t816));
            let tv2sigma25 = t6 * t820;
            acc_v2sigma2_5 = tv2sigma25;
            let t824 = f64x8::splat(1.0) / t356 / t19;
            let t825 = t358 * t138;
            let t828 = t357 * t138;
            let t831 = t134 * t134;
            let t832 = f64x8::splat(1.0) / t831;
            let t833 = t16 * t832;
            let t836 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t362 - f64x8::splat(6.0) * t833)));
            let t840 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t824 * t825 + f64x8::splat(4.0) / f64x8::splat(3.0) * t828 * t366 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t836));
            let t841 = t840 * t26;
            let t845 = t370 * t147;
            let t847 = t5 * t845 * t79;
            let t852 = t141 * t383;
            let t854 = t5 * t852 * t79;
            let t857 = t5 * t375 * t197;
            let t863 = f64x8::splat(1.0) / t146 / t134;
            let t864 = t25 * t863;
            let t867 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t864 * t79;
            let t869 = t5 * t384 * t197;
            let t872 = t5 * t148 * t447;
            let t875 = f64x8::splat(1.0) / t37 / t58;
            let t881 = f64x8::splat(1.0) / t36 / t71;
            let t882 = t56 * t881;
            let t886 = f64x8::splat(1.0) / t185;
            let t887 = t886 * t75;
            let t892 = f64x8::splat(1.0) / t37 / t771;
            let t893 = t184 * t892;
            let t895 = t408 * param_a;
            let t897 = t191 * t895 * t193;
            let t903 = t887 * param_a;
            let t909 = t71 * t71;
            let t911 = f64x8::splat(1.0) / t36 / t909;
            let t912 = t434 * t911;
            let t915 = t441 * t895 * t443;
            let t928 = t67 * t67;
            let t929 = f64x8::splat(1.0) / t928;
            let t930 = param_c3 * t929;
            let t931 = t184 * t56;
            let t932 = t930 * t931;
            let t933 = t909 * t152;
            let t936 = f64x8::splat(1.0) / t190 / t62;
            let t937 = f64x8::splat(1.0) / t933 * t936;
            let t938 = t937 * t895;
            let t941 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t34 * v_sigma0 * t875 * t47 + f64x8::splat(341.0) / f64x8::splat(972.0) * t160 * t882 * t165 - f64x8::splat(19.0) / f64x8::splat(162.0) * t405 * t887 * t408 + t404 * t893 * t897 / f64x8::splat(486.0) - f64x8::splat(209.0) / f64x8::splat(486.0) * t55 * t882 * t63 + f64x8::splat(797.0) / f64x8::splat(1944.0) * t173 * t903 - t172 * t893 * t424 / f64x8::splat(48.0) + t172 * t912 * t915 / f64x8::splat(2916.0) - f64x8::splat(5.0) / f64x8::splat(16.0) * t69 * t70 * t886 * t75 + f64x8::splat(1445.0) / f64x8::splat(31104.0) * t69 * t893 * t194 - f64x8::splat(35.0) / f64x8::splat(15552.0) * t69 * t912 * t444 + f64x8::splat(5.0) / f64x8::splat(23328.0) * t932 * t938;
            let t946 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t841 * t79 - f64x8::splat(3.0) / f64x8::splat(8.0) * t847 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t371 * t197 + t854 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t857 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t142 * t447 - t867 + t869 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t872 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t941));
            let t948 = f64x8::splat(1.0) / t453 / t88;
            let t949 = t455 * t205;
            let t952 = t454 * t205;
            let t955 = t85 * t832;
            let t958 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t362 - f64x8::splat(6.0) * t955)));
            let t962 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t948 * t949 + f64x8::splat(4.0) / f64x8::splat(3.0) * t952 * t461 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t958));
            let t963 = t962 * t26;
            let t967 = t465 * t147;
            let t969 = t5 * t967 * t129;
            let t971 = t208 * t383;
            let t973 = t5 * t971 * t129;
            let t975 = t92 * t863;
            let t978 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t975 * t129;
            let t980 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t963 * t129 - f64x8::splat(3.0) / f64x8::splat(8.0) * t969 + t973 / f64x8::splat(4.0) - t978));
            let tv3rho30 = f64x8::splat(3.0) * t452 + f64x8::splat(3.0) * t479 + t6 * (t946 + t980);
            acc_v3rho3_0 = tv3rho30;
            let t983 = f64x8::splat(2.0) * t505;
            let t984 = f64x8::splat(2.0) * t531;
            let t985 = t824 * t222;
            let t988 = t357 * t486;
            let t993 = f64x8::splat(2.0) * t362;
            let t994 = f64x8::splat(6.0) * t833;
            let t996 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t993 - t994)));
            let t1000 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t985 * t358 + f64x8::splat(8.0) / f64x8::splat(9.0) * t988 * t138 + f64x8::splat(4.0) / f64x8::splat(9.0) * t482 * t366 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t996));
            let t1001 = t1000 * t26;
            let t1005 = t490 * t147;
            let t1008 = t5 * t1005 * t79 / f64x8::splat(4.0);
            let t1012 = t225 * t383;
            let t1014 = t5 * t1012 * t79;
            let t1018 = t5 * t495 * t197 / f64x8::splat(4.0);
            let t1027 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1001 * t79 - t1008 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t491 * t197 + t1014 / f64x8::splat(12.0) - t1018 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t447 - t847 / f64x8::splat(8.0) + t854 / f64x8::splat(6.0) - t857 / f64x8::splat(4.0) - t867 + t869 / f64x8::splat(6.0) - t872 / f64x8::splat(8.0);
            let t1028 = ((t1).select(f64x8::splat(0.0), t1027));
            let t1029 = t948 * t233;
            let t1032 = t454 * t510;
            let t1037 = f64x8::splat(6.0) * t955;
            let t1039 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t993 - t1037)));
            let t1043 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1029 * t455 + f64x8::splat(8.0) / f64x8::splat(9.0) * t1032 * t205 + f64x8::splat(4.0) / f64x8::splat(9.0) * t506 * t461 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t1039));
            let t1044 = t1043 * t26;
            let t1048 = t514 * t147;
            let t1051 = t5 * t1048 * t129 / f64x8::splat(4.0);
            let t1052 = t236 * t383;
            let t1054 = t5 * t1052 * t129;
            let t1063 = t5 * t470 * t282 / f64x8::splat(4.0);
            let t1065 = t5 * t474 * t282;
            let t1068 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1044 * t129 - t1051 + t1054 / f64x8::splat(12.0) - t969 / f64x8::splat(8.0) + t973 / f64x8::splat(6.0) - t978 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t466 * t282 - t1063 + t1065 / f64x8::splat(12.0)));
            let tv3rho31 = t452 + t479 + t983 + t984 + t6 * (t1028 + t1068);
            acc_v3rho3_1 = tv3rho31;
            let t1071 = t824 * t536;
            let t1076 = t357 * t541;
            let t1080 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t993 - t994)));
            let t1084 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1071 * t138 + f64x8::splat(8.0) / f64x8::splat(9.0) * t482 * t486 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1076 * t138 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1080));
            let t1085 = t1084 * t26;
            let t1089 = t545 * t147;
            let t1091 = t5 * t1089 * t79;
            let t1100 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1085 * t79 - t1091 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t546 * t197 - t1008 + t1014 / f64x8::splat(6.0) - t1018 + t854 / f64x8::splat(12.0) - t867 + t869 / f64x8::splat(12.0)));
            let t1101 = t948 * t553;
            let t1106 = t454 * t558;
            let t1110 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t993 - t1037)));
            let t1114 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1101 * t205 + f64x8::splat(8.0) / f64x8::splat(9.0) * t506 * t510 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1106 * t205 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t1110));
            let t1115 = t1114 * t26;
            let t1119 = t562 * t147;
            let t1121 = t5 * t1119 * t129;
            let t1128 = t5 * t519 * t282;
            let t1136 = t5 * t213 * t625;
            let t1138 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1115 * t129 - t1121 / f64x8::splat(8.0) - t1051 + t1054 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t515 * t282 - t1128 / f64x8::splat(4.0) + t973 / f64x8::splat(12.0) - t978 - t1063 + t1065 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t625 - t1136 / f64x8::splat(8.0);
            let t1139 = ((t84).select(f64x8::splat(0.0), t1138));
            let tv3rho32 = t983 + t984 + t552 + t630 + t6 * (t1100 + t1139);
            acc_v3rho3_2 = tv3rho32;
            let t1144 = t536 * t222;
            let t1151 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t362 - f64x8::splat(6.0) * t833)));
            let t1155 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t824 * t1144 + f64x8::splat(4.0) / f64x8::splat(3.0) * t482 * t541 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1151));
            let t1156 = t1155 * t26;
            let t1163 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1156 * t79 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1091 + t1014 / f64x8::splat(4.0) - t867));
            let t1164 = t553 * t233;
            let t1171 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t362 - f64x8::splat(6.0) * t955)));
            let t1175 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t948 * t1164 + f64x8::splat(4.0) / f64x8::splat(3.0) * t506 * t558 + f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t1171));
            let t1176 = t1175 * t26;
            let t1192 = f64x8::splat(1.0) / t96 / t111;
            let t1198 = f64x8::splat(1.0) / t95 / t121;
            let t1199 = t109 * t1198;
            let t1203 = f64x8::splat(1.0) / t271;
            let t1204 = t1203 * t125;
            let t1209 = f64x8::splat(1.0) / t96 / t809;
            let t1210 = t270 * t1209;
            let t1213 = t277 * t895 * t193;
            let t1219 = t1204 * param_a;
            let t1225 = t121 * t121;
            let t1227 = f64x8::splat(1.0) / t95 / t1225;
            let t1228 = t613 * t1227;
            let t1231 = t620 * t895 * t443;
            let t1244 = t270 * t109;
            let t1245 = t930 * t1244;
            let t1246 = t1225 * t241;
            let t1249 = f64x8::splat(1.0) / t276 / t115;
            let t1250 = f64x8::splat(1.0) / t1246 * t1249;
            let t1251 = t1250 * t895;
            let t1254 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t34 * v_sigma2 * t1192 * t105 + f64x8::splat(341.0) / f64x8::splat(972.0) * t160 * t1199 * t252 - f64x8::splat(19.0) / f64x8::splat(162.0) * t585 * t1204 * t408 + t404 * t1210 * t1213 / f64x8::splat(486.0) - f64x8::splat(209.0) / f64x8::splat(486.0) * t55 * t1199 * t116 + f64x8::splat(797.0) / f64x8::splat(1944.0) * t259 * t1219 - t172 * t1210 * t603 / f64x8::splat(48.0) + t172 * t1228 * t1231 / f64x8::splat(2916.0) - f64x8::splat(5.0) / f64x8::splat(16.0) * t69 * t120 * t1203 * t125 + f64x8::splat(1445.0) / f64x8::splat(31104.0) * t69 * t1210 * t279 - f64x8::splat(35.0) / f64x8::splat(15552.0) * t69 * t1228 * t622 + f64x8::splat(5.0) / f64x8::splat(23328.0) * t1245 * t1251;
            let t1259 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1176 * t129 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1121 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t563 * t282 + t1054 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1128 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t237 * t625 - t978 + t1065 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1136 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t1254));
            let tv3rho33 = f64x8::splat(3.0) * t552 + f64x8::splat(3.0) * t630 + t6 * (t1163 + t1259);
            acc_v3rho3_3 = tv3rho33;
            let t1267 = t5 * t375 * t317;
            let t1274 = t5 * t384 * t317 / f64x8::splat(12.0);
            let t1276 = t5 * t148 * t676;
            let t1282 = t399 * t63;
            let t1286 = t404 * t406;
            let t1287 = t75 * t408;
            let t1288 = t1287 * t56;
            let t1291 = t70 * t420;
            let t1304 = t184 * t437;
            let t1318 = t930 * t434;
            let t1319 = t909 * t35;
            let t1321 = f64x8::splat(1.0) / t1319 * t936;
            let t1322 = t1321 * t895;
            let t1325 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t33 * t392 * t47 - f64x8::splat(65.0) / f64x8::splat(648.0) * t160 * t1282 * t644 + f64x8::splat(17.0) / f64x8::splat(432.0) * t1286 * t1288 - t404 * t1291 * t897 / f64x8::splat(1296.0) + f64x8::splat(19.0) / f64x8::splat(162.0) * t55 * v_sigma0 * t399 * t63 - f64x8::splat(167.0) / f64x8::splat(1296.0) * t301 * t415 + f64x8::splat(25.0) / f64x8::splat(3456.0) * t172 * t1291 * t424 - t172 * t1304 * t915 / f64x8::splat(7776.0) + f64x8::splat(3.0) / f64x8::splat(32.0) * t69 * t56 * t406 * t75 - f64x8::splat(317.0) / f64x8::splat(20736.0) * t69 * t1291 * t194 + f64x8::splat(11.0) / f64x8::splat(13824.0) * t69 * t1304 * t444 - f64x8::splat(5.0) / f64x8::splat(62208.0) * t1318 * t1322;
            let t1330 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t371 * t317 - t1267 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t142 * t676 + t1274 - t1276 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1325));
            let tv3rho2sigma0 = t6 * t1330 + f64x8::splat(2.0) * t681;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1337 = t5 * t470 * t349;
            let t1341 = t5 * t474 * t349 / f64x8::splat(12.0);
            let t1343 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t466 * t349 - t1337 / f64x8::splat(4.0) + t1341));
            let tv3rho2sigma2 = t6 * t1343 + f64x8::splat(2.0) * t690;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1349 = t5 * t495 * t317;
            let t1357 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t491 * t317 - t1349 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t676 - t1267 / f64x8::splat(8.0) + t1274 - t1276 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1357 + t681 + t696;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1363 = t5 * t519 * t349;
            let t1370 = t5 * t213 * t738;
            let t1373 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t515 * t349 - t1363 / f64x8::splat(8.0) - t1337 / f64x8::splat(8.0) + t1341 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t738 - t1370 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1373 + t690 + t743;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1381 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t546 * t317 - t1349 / f64x8::splat(4.0) + t1274));
            let tv3rho2sigma6 = t6 * t1381 + f64x8::splat(2.0) * t696;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1396 = t580 * t116;
            let t1400 = t404 * t586;
            let t1401 = t125 * t408;
            let t1402 = t1401 * t109;
            let t1405 = t120 * t599;
            let t1418 = t270 * t616;
            let t1432 = t930 * t613;
            let t1433 = t1225 * t94;
            let t1435 = f64x8::splat(1.0) / t1433 * t1249;
            let t1436 = t1435 * t895;
            let t1439 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t33 * t573 * t105 - f64x8::splat(65.0) / f64x8::splat(648.0) * t160 * t1396 * t706 + f64x8::splat(17.0) / f64x8::splat(432.0) * t1400 * t1402 - t404 * t1405 * t1213 / f64x8::splat(1296.0) + f64x8::splat(19.0) / f64x8::splat(162.0) * t55 * v_sigma2 * t580 * t116 - f64x8::splat(167.0) / f64x8::splat(1296.0) * t333 * t594 + f64x8::splat(25.0) / f64x8::splat(3456.0) * t172 * t1405 * t603 - t172 * t1418 * t1231 / f64x8::splat(7776.0) + f64x8::splat(3.0) / f64x8::splat(32.0) * t69 * t109 * t586 * t125 - f64x8::splat(317.0) / f64x8::splat(20736.0) * t69 * t1405 * t279 + f64x8::splat(11.0) / f64x8::splat(13824.0) * t69 * t1418 * t622 - f64x8::splat(5.0) / f64x8::splat(62208.0) * t1432 * t1436;
            let t1444 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t563 * t349 - t1363 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t237 * t738 + t1341 - t1370 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t1439));
            let tv3rho2sigma8 = t6 * t1444 + f64x8::splat(2.0) * t743;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1451 = t5 * t148 * t778 / f64x8::splat(8.0);
            let t1456 = t1287 * v_sigma0;
            let t1459 = t56 * t187;
            let t1469 = t75 * param_a * v_sigma0;
            let t1475 = t70 * t671;
            let t1489 = t930 * t184;
            let t1490 = t909 * v_rho0;
            let t1492 = f64x8::splat(1.0) / t1490 * t936;
            let t1493 = t1492 * t895;
            let t1496 = t160 * t643 * param_a / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(432.0) * t404 * t175 * t1456 + t404 * t1459 * t897 / f64x8::splat(3456.0) - t52 * t54 * t163 * t63 / f64x8::splat(54.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * t172 * t175 * t1469 - t172 * t1459 * t424 / f64x8::splat(432.0) + t172 * t1475 * t915 / f64x8::splat(20736.0) - t69 * v_sigma0 * t175 * t75 / f64x8::splat(48.0) + f64x8::splat(5.0) / f64x8::splat(1152.0) * t69 * t1459 * t194 - f64x8::splat(11.0) / f64x8::splat(41472.0) * t69 * t1475 * t444 + f64x8::splat(5.0) / f64x8::splat(165888.0) * t1489 * t1493;
            let t1501 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t778 - t1451 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1496));
            let tv3rhosigma20 = t6 * t1501 + t782;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1508 = t5 * t213 * t816 / f64x8::splat(8.0);
            let t1510 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t816 - t1508));
            let tv3rhosigma25 = t6 * t1510 + t820;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1516 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t778 - t1451));
            let tv3rhosigma26 = t6 * t1516 + t782;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1525 = t1401 * v_sigma2;
            let t1528 = t109 * t273;
            let t1538 = t125 * param_a * v_sigma2;
            let t1544 = t120 * t733;
            let t1558 = t930 * t270;
            let t1559 = t1225 * v_rho1;
            let t1561 = f64x8::splat(1.0) / t1559 * t1249;
            let t1562 = t1561 * t895;
            let t1565 = t160 * t705 * param_a / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(432.0) * t404 * t261 * t1525 + t404 * t1528 * t1213 / f64x8::splat(3456.0) - t52 * t54 * t250 * t116 / f64x8::splat(54.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * t172 * t261 * t1538 - t172 * t1528 * t603 / f64x8::splat(432.0) + t172 * t1544 * t1231 / f64x8::splat(20736.0) - t69 * v_sigma2 * t261 * t125 / f64x8::splat(48.0) + f64x8::splat(5.0) / f64x8::splat(1152.0) * t69 * t1528 * t279 - f64x8::splat(11.0) / f64x8::splat(41472.0) * t69 * t1544 * t622 + f64x8::splat(5.0) / f64x8::splat(165888.0) * t1558 * t1562;
            let t1570 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t237 * t816 - t1508 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t1565));
            let tv3rhosigma211 = t6 * t1570 + t820;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1574 = v_sigma0 * t312;
            let t1583 = t56 * t773;
            let t1595 = t930 * t70;
            let t1597 = f64x8::splat(1.0) / t909 * t936;
            let t1598 = t1597 * t895;
            let t1601 = t404 * t750 / f64x8::splat(384.0) - t404 * t1574 * t897 / f64x8::splat(9216.0) - t172 * t303 / f64x8::splat(192.0) + t172 * t1574 * t424 / f64x8::splat(1536.0) - t172 * t1583 * t915 / f64x8::splat(55296.0) + t69 * t302 / f64x8::splat(384.0) - t69 * t1574 * t194 / f64x8::splat(1024.0) + t69 * t1583 * t444 / f64x8::splat(12288.0) - f64x8::splat(5.0) / f64x8::splat(442368.0) * t1595 * t1598;
            let t1605 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1601));
            let tv3sigma30 = t6 * t1605;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1608 = v_sigma2 * t344;
            let t1617 = t109 * t811;
            let t1629 = t930 * t120;
            let t1631 = f64x8::splat(1.0) / t1225 * t1249;
            let t1632 = t1631 * t895;
            let t1635 = t404 * t788 / f64x8::splat(384.0) - t404 * t1608 * t1213 / f64x8::splat(9216.0) - t172 * t335 / f64x8::splat(192.0) + t172 * t1608 * t603 / f64x8::splat(1536.0) - t172 * t1617 * t1231 / f64x8::splat(55296.0) + t69 * t334 / f64x8::splat(384.0) - t69 * t1608 * t279 / f64x8::splat(1024.0) + t69 * t1617 * t622 / f64x8::splat(12288.0) - f64x8::splat(5.0) / f64x8::splat(442368.0) * t1629 * t1632;
            let t1639 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t1635));
            let tv3sigma39 = t6 * t1639;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
