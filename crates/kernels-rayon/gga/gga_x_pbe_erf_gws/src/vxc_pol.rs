//! GGA_X_PBE_ERF_GWS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`
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
pub fn gga_x_pbe_erf_gws_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_ax: f64,
    param_b_PBE: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_ax = f64x8::splat(param_ax);
    let param_b_PBE = f64x8::splat(param_b_PBE);
    let param_kappa = f64x8::splat(param_kappa);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRTPI);
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = param_hyb_omega_0 * param_hyb_omega_0;
            let t5 = param_ax * t4;
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = t5 * t6;
            let t8 = t2 * f64x8::splat(M_PI);
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = ((t10).select(t11, t12));
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t9 * t15;
            let t17 = (simd::cbrt(v_rho0));
            let t18 = t17 * t17;
            let t19 = f64x8::splat(1.0) / t18;
            let t23 = (simd::exp(-t7 * t16 * t19 / f64x8::splat(12.0)));
            let t25 = param_b_PBE * t23 * v_sigma0;
            let t26 = t6 * t12;
            let t27 = param_kappa + f64x8::splat(1.0);
            let t28 = t6 * t6;
            let t29 = t2 * t2;
            let t31 = t28 / t29;
            let t32 = f64x8::splat(1.0) / t17;
            let t34 = f64x8::splat(1.0) / t13;
            let t37 = t31 * param_hyb_omega_0 * t32 * t34 / f64x8::splat(6.0);
            let t38 = (t37).simd_lt(f64x8::splat(0.05));
            let t39 = t14 * t14;
            let t40 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t41 = t29 * t40;
            let t42 = t39 * t41;
            let t43 = t17 * v_rho0;
            let t44 = t42 * t43;
            let t46 = t14 * t8;
            let t49 = t46 * t6 * t18 * t4;
            let t51 = f64x8::splat(7.0) * t44 - f64x8::splat(6.0) * t49;
            let t52 = t14 * t13;
            let t53 = f64x8::splat(1.0) / param_hyb_omega_0;
            let t54 = t53 * t6;
            let t55 = t29 * t13;
            let t58 = (simd::erf(t54 * t55 * t17));
            let t59 = t52 * t58;
            let t60 = ((f64x8::splat(M_PI)).sqrt());
            let t61 = t60 * t40;
            let t69 = t4 * t4;
            let t71 = f64x8::splat(6.0) * t28 * t69;
            let t72 = -f64x8::splat(36.0) * t59 * t61 * t28 * v_rho0 * param_hyb_omega_0 + f64x8::splat(81.0) * t44 + f64x8::splat(54.0) * t49 - t71;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = (f64x8::splat(10000000000.0)).simd_lt(t37);
            let t76 = t40 * t40;
            let t77 = v_rho0 * v_rho0;
            let t79 = t39 * t14;
            let t82 = t8 * t28;
            let t87 = t41 * t6;
            let t93 = t69 * t4;
            let t94 = f64x8::splat(1.0) / t93;
            let t98 = f64x8::splat(1.0) / t4;
            let t99 = t98 * t28;
            let t101 = t99 * t46 * t18;
            let t102 = (simd::exp(t101));
            let t103 = t102 * t6;
            let t104 = t103 * t14;
            let t106 = t8 * t18 * t4;
            let t110 = t102 * t28;
            let t114 = (f64x8::splat(7.0) * t104 * t106 - f64x8::splat(6.0) * t110 * t69 + f64x8::splat(6.0) * t44 + f64x8::splat(11.0) * t49 + t71) * t8;
            let t116 = t14 * t6;
            let t117 = t39 * t102;
            let t118 = t41 * t28;
            let t123 = t52 * t102 * t58;
            let t124 = t61 * t6;
            let t129 = t14 * t102;
            let t138 = f64x8::splat(2.0) * t69 * t6;
            let t139 = f64x8::splat(12.0) * t123 * t124 * v_rho0 * param_hyb_omega_0 - f64x8::splat(9.0) * t117 * t118 * t43 + f64x8::splat(12.0) * t46 * t18 * t4 + f64x8::splat(2.0) * t103 * t69 - f64x8::splat(18.0) * t129 * t106 - t138;
            let t142 = t116 * t98 / t139;
            let t145 = ((t38).select(t51 * t73, (t75).select((f64x8::splat(2800.0) * t82 * t18 * t69 * t14 - f64x8::splat(140.0) * t87 * t43 * t4 * t39 - f64x8::splat(1863.0) * t76 * t77 * t79) * t94 / f64x8::splat(50400.0), -t114 * t18 * t142 / f64x8::splat(9.0))));
            let t146 = t27 * t145;
            let t150 = t18 * t77;
            let t152 = param_kappa * t150 * t8;
            let t153 = f64x8::splat(27.0) / f64x8::splat(56.0) * t25 * t26 * t146 + t152;
            let t154 = t3 * t153;
            let t155 = t154 * t6;
            let t158 = ((t10).select(t11 * zeta_threshold, f64x8::splat(2.0) * t12));
            let t159 = t158 * t43;
            let t160 = (f64x8::splat(1.35)).simd_le(t37);
            let t161 = (f64x8::splat(1.35)).simd_lt(t37);
            let t162 = ((t161).select(t37, f64x8::splat(1.35)));
            let t163 = t162 * t162;
            let t164 = t163 * t163;
            let t165 = t164 * t163;
            let t166 = t164 * t164;
            let t169 = t166 * t164;
            let t171 = t166 * t163;
            let t177 = f64x8::splat(24088884019200.0) * t166 * t165 + f64x8::splat(19448.0) * t163 - f64x8::splat(807840.0) * t164 + f64x8::splat(30551040.0) * t165 - f64x8::splat(1045524480.0) * t166 - f64x8::splat(903333150720.0) * t169 + f64x8::splat(32261898240.0) * t171 - f64x8::splat(429.0);
            let t178 = t166 * t166;
            let t179 = f64x8::splat(1.0) / t178;
            let t182 = ((t161).select(f64x8::splat(1.35), t37));
            let t183 = t182 * t182;
            let t184 = t183 * t183;
            let t187 = f64x8::splat(32.0) * t184 - f64x8::splat(16.0) * t183;
            let t190 = (simd::exp(-f64x8::splat(1.0) / t183 / f64x8::splat(4.0)));
            let t194 = f64x8::splat(1.0) / t182;
            let t196 = (simd::erf(t194 / f64x8::splat(2.0)));
            let t197 = t60 * t196;
            let t202 = ((t160).select(t177 * t179 / f64x8::splat(867199824691200.0), t187 * t190 / f64x8::splat(3.0) - f64x8::splat(32.0) / f64x8::splat(3.0) * t184 - f64x8::splat(8.0) / f64x8::splat(3.0) * t197 * t182 + f64x8::splat(8.0) * t183 + f64x8::splat(1.0)));
            let t203 = param_b_PBE * t145;
            let t206 = v_sigma0 * t12 * t6;
            let t210 = f64x8::splat(216.0) * t203 * t23 * t206 + f64x8::splat(448.0) * t152;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t202 * t211;
            let t213 = t159 * t212;
            let t216 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(168.0) * t155 * t213));
            let t217 = (v_rho1).simd_le(dens_threshold);
            let t218 = (simd::cbrt(v_rho1));
            let t219 = t218 * t218;
            let t220 = f64x8::splat(1.0) / t219;
            let t224 = (simd::exp(-t7 * t16 * t220 / f64x8::splat(12.0)));
            let t226 = param_b_PBE * t224 * v_sigma2;
            let t227 = f64x8::splat(1.0) / t218;
            let t231 = t31 * param_hyb_omega_0 * t227 * t34 / f64x8::splat(6.0);
            let t232 = (t231).simd_lt(f64x8::splat(0.05));
            let t233 = t218 * v_rho1;
            let t234 = t42 * t233;
            let t238 = t46 * t6 * t219 * t4;
            let t240 = f64x8::splat(7.0) * t234 - f64x8::splat(6.0) * t238;
            let t243 = (simd::erf(t54 * t55 * t218));
            let t244 = t52 * t243;
            let t252 = -f64x8::splat(36.0) * t244 * t61 * t28 * v_rho1 * param_hyb_omega_0 + f64x8::splat(81.0) * t234 + f64x8::splat(54.0) * t238 - t71;
            let t253 = f64x8::splat(1.0) / t252;
            let t255 = (f64x8::splat(10000000000.0)).simd_lt(t231);
            let t256 = v_rho1 * v_rho1;
            let t273 = t99 * t46 * t219;
            let t274 = (simd::exp(t273));
            let t275 = t274 * t219;
            let t277 = t46 * t4;
            let t281 = t274 * t28;
            let t285 = (f64x8::splat(7.0) * t275 * t6 * t277 - f64x8::splat(6.0) * t281 * t69 + f64x8::splat(6.0) * t234 + f64x8::splat(11.0) * t238 + t71) * t8;
            let t287 = t39 * t274;
            let t292 = t52 * t274 * t243;
            let t297 = t14 * t274;
            let t305 = t274 * t6;
            let t308 = f64x8::splat(12.0) * t292 * t124 * v_rho1 * param_hyb_omega_0 - f64x8::splat(18.0) * t297 * t8 * t219 * t4 - f64x8::splat(9.0) * t287 * t118 * t233 + f64x8::splat(12.0) * t46 * t219 * t4 + f64x8::splat(2.0) * t305 * t69 - t138;
            let t311 = t116 * t98 / t308;
            let t314 = ((t232).select(t240 * t253, (t255).select((f64x8::splat(2800.0) * t82 * t219 * t69 * t14 - f64x8::splat(140.0) * t87 * t233 * t4 * t39 - f64x8::splat(1863.0) * t76 * t256 * t79) * t94 / f64x8::splat(50400.0), -t285 * t219 * t311 / f64x8::splat(9.0))));
            let t315 = t27 * t314;
            let t319 = t219 * t256;
            let t321 = param_kappa * t319 * t8;
            let t322 = f64x8::splat(27.0) / f64x8::splat(56.0) * t226 * t26 * t315 + t321;
            let t323 = t3 * t322;
            let t324 = t323 * t6;
            let t325 = t158 * t233;
            let t326 = (f64x8::splat(1.35)).simd_le(t231);
            let t327 = (f64x8::splat(1.35)).simd_lt(t231);
            let t328 = ((t327).select(t231, f64x8::splat(1.35)));
            let t329 = t328 * t328;
            let t330 = t329 * t329;
            let t331 = t330 * t329;
            let t332 = t330 * t330;
            let t335 = t332 * t330;
            let t337 = t332 * t329;
            let t343 = f64x8::splat(24088884019200.0) * t332 * t331 + f64x8::splat(19448.0) * t329 - f64x8::splat(807840.0) * t330 + f64x8::splat(30551040.0) * t331 - f64x8::splat(1045524480.0) * t332 - f64x8::splat(903333150720.0) * t335 + f64x8::splat(32261898240.0) * t337 - f64x8::splat(429.0);
            let t344 = t332 * t332;
            let t345 = f64x8::splat(1.0) / t344;
            let t348 = ((t327).select(f64x8::splat(1.35), t231));
            let t349 = t348 * t348;
            let t350 = t349 * t349;
            let t353 = f64x8::splat(32.0) * t350 - f64x8::splat(16.0) * t349;
            let t356 = (simd::exp(-f64x8::splat(1.0) / t349 / f64x8::splat(4.0)));
            let t360 = f64x8::splat(1.0) / t348;
            let t362 = (simd::erf(t360 / f64x8::splat(2.0)));
            let t368 = ((t326).select(t343 * t345 / f64x8::splat(867199824691200.0), t353 * t356 / f64x8::splat(3.0) - f64x8::splat(32.0) / f64x8::splat(3.0) * t350 - f64x8::splat(8.0) / f64x8::splat(3.0) * t362 * t348 * t60 + f64x8::splat(8.0) * t349 + f64x8::splat(1.0)));
            let t369 = param_b_PBE * t314;
            let t372 = v_sigma2 * t12 * t6;
            let t376 = f64x8::splat(216.0) * t369 * t224 * t372 + f64x8::splat(448.0) * t321;
            let t377 = f64x8::splat(1.0) / t376;
            let t378 = t368 * t377;
            let t379 = t325 * t378;
            let t382 = ((t217).select(f64x8::splat(0.0), -f64x8::splat(168.0) * t324 * t379));
            let tzk0 = (t216 + t382) / (v_rho0 + v_rho1);
            acc_zk = tzk0;
            let t390 = param_b_PBE * param_ax * t4 * t28 * t9 * t15;
            let t391 = t18 * v_rho0;
            let t392 = f64x8::splat(1.0) / t391;
            let t394 = t392 * t23 * v_sigma0;
            let t395 = t12 * t27;
            let t396 = t395 * t145;
            let t400 = t42 * t17;
            let t402 = t6 * t32;
            let t404 = t46 * t402 * t4;
            let t406 = f64x8::splat(28.0) / f64x8::splat(3.0) * t400 - f64x8::splat(4.0) * t404;
            let t408 = t72 * t72;
            let t409 = f64x8::splat(1.0) / t408;
            let t410 = t51 * t409;
            let t411 = (simd::exp(-t101));
            let t416 = t61 * t28 * param_hyb_omega_0;
            let t421 = -f64x8::splat(72.0) * t42 * t411 * t17 - f64x8::splat(36.0) * t59 * t416 + f64x8::splat(108.0) * t400 + f64x8::splat(36.0) * t404;
            let t424 = t76 * v_rho0;
            let t443 = t8 * t32 * t4;
            let t448 = (f64x8::splat(8.0) * t400 + f64x8::splat(14.0) * t42 * t17 * t102 - f64x8::splat(22.0) / f64x8::splat(3.0) * t104 * t443 + f64x8::splat(22.0) / f64x8::splat(3.0) * t404) * t8;
            let t455 = t18 * t14;
            let t456 = t114 * t455;
            let t457 = t6 * t98;
            let t458 = t139 * t139;
            let t459 = f64x8::splat(1.0) / t458;
            let t460 = t79 * t98;
            let t461 = t460 * t6;
            let t470 = t40 * f64x8::splat(M_PI);
            let t471 = (simd::pow(f64x8::splat(M_PI), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t472 = t471 * t471;
            let t473 = t472 * t472;
            let t474 = t473 * t471;
            let t476 = t39 * t13 * t53 * t474 * t470;
            let t481 = t117 * t41;
            let t482 = t411 * t28;
            let t486 = t124 * param_hyb_omega_0;
            let t494 = f64x8::splat(24.0) * t476 * t18 * t102 * t58 - f64x8::splat(18.0) * t461 * t424 * t102 - f64x8::splat(24.0) * t117 * t118 * t17 + f64x8::splat(8.0) * t481 * t482 * t17 + f64x8::splat(8.0) * t46 * t32 * t4 + f64x8::splat(12.0) * t123 * t486 - f64x8::splat(8.0) * t129 * t443;
            let t496 = t457 * t459 * t494;
            let t500 = ((t38).select(t406 * t73 - t410 * t421, (t75).select((-f64x8::splat(3726.0) * t424 * t79 + f64x8::splat(5600.0) / f64x8::splat(3.0) * t82 * t32 * t69 * t14 - f64x8::splat(560.0) / f64x8::splat(3.0) * t87 * t17 * t4 * t39) * t94 / f64x8::splat(50400.0), -t448 * t18 * t142 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t114 * t32 * t142 + t456 * t496 / f64x8::splat(9.0))));
            let t501 = t27 * t500;
            let t506 = param_kappa * t391 * t8;
            let t508 = f64x8::splat(3.0) / f64x8::splat(112.0) * t390 * t394 * t396 + f64x8::splat(27.0) / f64x8::splat(56.0) * t25 * t26 * t501 + f64x8::splat(8.0) / f64x8::splat(3.0) * t506;
            let t509 = t3 * t508;
            let t510 = t509 * t6;
            let t513 = t158 * t17;
            let t514 = t513 * t212;
            let t517 = t164 * t162;
            let t518 = t166 * t517;
            let t519 = f64x8::splat(1.0) / t43;
            let t523 = t31 * param_hyb_omega_0 * t519 * t34 / f64x8::splat(18.0);
            let t524 = ((t161).select(-t523, f64x8::splat(0.0)));
            let t527 = t163 * t162;
            let t528 = t166 * t527;
            let t531 = t166 * t162;
            let t534 = t164 * t527;
            let t543 = f64x8::splat(38896.0) * t162 * t524 + f64x8::splat(183306240.0) * t517 * t524 + f64x8::splat(337244376268800.0) * t518 * t524 - f64x8::splat(3231360.0) * t527 * t524 - f64x8::splat(10839997808640.0) * t528 * t524 + f64x8::splat(322618982400.0) * t531 * t524 - f64x8::splat(8364195840.0) * t534 * t524;
            let t547 = f64x8::splat(1.0) / t178 / t162;
            let t548 = t177 * t547;
            let t552 = t183 * t182;
            let t553 = ((t161).select(f64x8::splat(0.0), -t523));
            let t554 = t552 * t553;
            let t556 = t182 * t553;
            let t558 = f64x8::splat(128.0) * t554 - f64x8::splat(32.0) * t556;
            let t561 = f64x8::splat(1.0) / t552;
            let t562 = t187 * t561;
            let t563 = t553 * t190;
            let t567 = t190 * t194;
            let t574 = ((t160).select(t543 * t179 / f64x8::splat(867199824691200.0) - t548 * t524 / f64x8::splat(54199989043200.0), t558 * t190 / f64x8::splat(3.0) + t562 * t563 / f64x8::splat(6.0) - f64x8::splat(128.0) / f64x8::splat(3.0) * t554 + f64x8::splat(8.0) / f64x8::splat(3.0) * t567 * t553 - f64x8::splat(8.0) / f64x8::splat(3.0) * t197 * t553 + f64x8::splat(16.0) * t556));
            let t575 = t574 * t211;
            let t576 = t159 * t575;
            let t579 = t6 * t158;
            let t580 = t154 * t579;
            let t581 = t43 * t202;
            let t582 = t210 * t210;
            let t583 = f64x8::splat(1.0) / t582;
            let t584 = param_b_PBE * t500;
            let t588 = t5 * t28;
            let t589 = t203 * t588;
            let t592 = t23 * v_sigma0 * t12;
            let t593 = t16 * t392 * t592;
            let t597 = f64x8::splat(216.0) * t584 * t23 * t206 + f64x8::splat(12.0) * t589 * t593 + f64x8::splat(3584.0) / f64x8::splat(3.0) * t506;
            let t598 = t583 * t597;
            let t599 = t581 * t598;
            let tvrho0 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(224.0) * t155 * t514 - f64x8::splat(168.0) * t155 * t576 - f64x8::splat(168.0) * t510 * t213 + f64x8::splat(168.0) * t580 * t599));
            acc_vrho_0 = tvrho0;
            let t603 = t219 * v_rho1;
            let t604 = f64x8::splat(1.0) / t603;
            let t606 = t604 * t224 * v_sigma2;
            let t607 = t395 * t314;
            let t611 = t42 * t218;
            let t613 = t6 * t227;
            let t615 = t46 * t613 * t4;
            let t617 = f64x8::splat(28.0) / f64x8::splat(3.0) * t611 - f64x8::splat(4.0) * t615;
            let t619 = t252 * t252;
            let t620 = f64x8::splat(1.0) / t619;
            let t621 = t240 * t620;
            let t622 = (simd::exp(-t273));
            let t630 = -f64x8::splat(72.0) * t42 * t622 * t218 - f64x8::splat(36.0) * t244 * t416 + f64x8::splat(108.0) * t611 + f64x8::splat(36.0) * t615;
            let t633 = t76 * v_rho1;
            let t651 = t274 * t227;
            let t652 = t651 * t6;
            let t657 = (f64x8::splat(8.0) * t611 + f64x8::splat(14.0) * t42 * t218 * t274 - f64x8::splat(22.0) / f64x8::splat(3.0) * t652 * t277 + f64x8::splat(22.0) / f64x8::splat(3.0) * t615) * t8;
            let t664 = t219 * t14;
            let t665 = t285 * t664;
            let t666 = t308 * t308;
            let t667 = f64x8::splat(1.0) / t666;
            let t677 = t287 * t41;
            let t678 = t622 * t28;
            let t691 = -f64x8::splat(8.0) * t297 * t8 * t227 * t4 - f64x8::splat(24.0) * t287 * t118 * t218 + f64x8::splat(8.0) * t677 * t678 * t218 + f64x8::splat(8.0) * t46 * t227 * t4 + f64x8::splat(24.0) * t476 * t275 * t243 - f64x8::splat(18.0) * t461 * t633 * t274 + f64x8::splat(12.0) * t292 * t486;
            let t693 = t457 * t667 * t691;
            let t697 = ((t232).select(t617 * t253 - t621 * t630, (t255).select((-f64x8::splat(3726.0) * t633 * t79 + f64x8::splat(5600.0) / f64x8::splat(3.0) * t82 * t227 * t69 * t14 - f64x8::splat(560.0) / f64x8::splat(3.0) * t87 * t218 * t4 * t39) * t94 / f64x8::splat(50400.0), -t657 * t219 * t311 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t285 * t227 * t311 + t665 * t693 / f64x8::splat(9.0))));
            let t698 = t27 * t697;
            let t703 = param_kappa * t603 * t8;
            let t705 = f64x8::splat(3.0) / f64x8::splat(112.0) * t390 * t606 * t607 + f64x8::splat(27.0) / f64x8::splat(56.0) * t226 * t26 * t698 + f64x8::splat(8.0) / f64x8::splat(3.0) * t703;
            let t706 = t3 * t705;
            let t707 = t706 * t6;
            let t710 = t158 * t218;
            let t711 = t710 * t378;
            let t714 = t330 * t328;
            let t715 = t332 * t714;
            let t716 = f64x8::splat(1.0) / t233;
            let t720 = t31 * param_hyb_omega_0 * t716 * t34 / f64x8::splat(18.0);
            let t721 = ((t327).select(-t720, f64x8::splat(0.0)));
            let t724 = t329 * t328;
            let t725 = t332 * t724;
            let t728 = t332 * t328;
            let t731 = t330 * t724;
            let t740 = f64x8::splat(38896.0) * t328 * t721 + f64x8::splat(183306240.0) * t714 * t721 + f64x8::splat(337244376268800.0) * t715 * t721 - f64x8::splat(3231360.0) * t724 * t721 - f64x8::splat(10839997808640.0) * t725 * t721 + f64x8::splat(322618982400.0) * t728 * t721 - f64x8::splat(8364195840.0) * t731 * t721;
            let t744 = f64x8::splat(1.0) / t344 / t328;
            let t745 = t343 * t744;
            let t749 = t349 * t348;
            let t750 = ((t327).select(f64x8::splat(0.0), -t720));
            let t751 = t749 * t750;
            let t753 = t348 * t750;
            let t755 = f64x8::splat(128.0) * t751 - f64x8::splat(32.0) * t753;
            let t758 = f64x8::splat(1.0) / t749;
            let t759 = t353 * t758;
            let t760 = t750 * t356;
            let t764 = t356 * t360;
            let t772 = ((t326).select(t740 * t345 / f64x8::splat(867199824691200.0) - t745 * t721 / f64x8::splat(54199989043200.0), t755 * t356 / f64x8::splat(3.0) + t759 * t760 / f64x8::splat(6.0) - f64x8::splat(128.0) / f64x8::splat(3.0) * t751 + f64x8::splat(8.0) / f64x8::splat(3.0) * t764 * t750 - f64x8::splat(8.0) / f64x8::splat(3.0) * t362 * t750 * t60 + f64x8::splat(16.0) * t753));
            let t773 = t772 * t377;
            let t774 = t325 * t773;
            let t777 = t323 * t579;
            let t778 = t233 * t368;
            let t779 = t376 * t376;
            let t780 = f64x8::splat(1.0) / t779;
            let t781 = param_b_PBE * t697;
            let t785 = t369 * t588;
            let t788 = t224 * v_sigma2 * t12;
            let t789 = t16 * t604 * t788;
            let t793 = f64x8::splat(216.0) * t781 * t224 * t372 + f64x8::splat(12.0) * t785 * t789 + f64x8::splat(3584.0) / f64x8::splat(3.0) * t703;
            let t794 = t780 * t793;
            let t795 = t778 * t794;
            let tvrho1 = ((t217).select(f64x8::splat(0.0), -f64x8::splat(224.0) * t324 * t711 - f64x8::splat(168.0) * t324 * t774 - f64x8::splat(168.0) * t707 * t379 + f64x8::splat(168.0) * t777 * t795));
            acc_vrho_1 = tvrho1;
            let t799 = t3 * param_b_PBE;
            let t800 = t23 * t12;
            let t802 = t799 * t800 * t28;
            let t803 = t146 * t158;
            let t804 = t581 * t211;
            let t808 = t28 * t158;
            let t809 = t808 * t43;
            let t810 = t154 * t809;
            let t811 = t202 * t583;
            let t812 = t811 * param_b_PBE;
            let t814 = t145 * t23 * t12;
            let t815 = t812 * t814;
            let tvsigma0 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(81.0) * t802 * t803 * t804 + f64x8::splat(36288.0) * t810 * t815));
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t819 = t224 * t12;
            let t821 = t799 * t819 * t28;
            let t822 = t315 * t158;
            let t823 = t778 * t377;
            let t827 = t808 * t233;
            let t828 = t323 * t827;
            let t829 = t368 * t780;
            let t830 = t829 * param_b_PBE;
            let t832 = t314 * t224 * t12;
            let t833 = t830 * t832;
            let tvsigma2 = ((t217).select(f64x8::splat(0.0), -f64x8::splat(81.0) * t821 * t822 * t823 + f64x8::splat(36288.0) * t828 * t833));
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
