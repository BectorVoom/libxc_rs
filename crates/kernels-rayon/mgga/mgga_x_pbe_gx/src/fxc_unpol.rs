//! MGGA_X_PBE_GX fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_pbe_gx_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_CBRT4);
            let t26 = f64x8::splat(8.0) / f64x8::splat(27.0) * t21 * t22 * t24;
            let t27 = t21 * t21;
            let t28 = v_tau * t27;
            let t29 = t20 * t20;
            let t31 = f64x8::splat(1.0) / t29 / v_rho;
            let t33 = v_sigma * t27;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t29 / t34;
            let t37 = t33 * t36;
            let t39 = t28 * t31 - t37 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t39 * t40 * t45;
            let t48 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t46;
            let t50 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t46;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = f64x8::splat(1.0) - t26;
            let t54 = t48 * t51 * t53;
            let t57 = t26 + f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t54;
            let t58 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46;
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = ((t59).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t62 = f64x8::splat(1.0) + t58;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) + f64x8::splat(0.148) * t59 * t63;
            let t67 = -t59;
            let t68 = ((t67).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t70 = t57 * t60 + t66 * t68;
            let t73 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t37;
            let t74 = f64x8::splat(1.0) / t73;
            let t78 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t70 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t79 = f64x8::splat(1.0) / t29;
            let t86 = t34 * v_rho;
            let t88 = f64x8::splat(1.0) / t29 / t86;
            let t91 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t28 * t36 + t33 * t88 / f64x8::splat(3.0);
            let t92 = t91 * t40;
            let t93 = t92 * t45;
            let t96 = t40 * t40;
            let t97 = t39 * t96;
            let t99 = f64x8::splat(1.0) / t43 / t42;
            let t100 = t97 * t99;
            let t102 = t91 * t51 * t53;
            let t105 = t50 * t50;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t48 * t106;
            let t108 = t53 * t91;
            let t109 = t107 * t108;
            let t112 = f64x8::splat(5.0) / f64x8::splat(9.0) * t93 * t54 - f64x8::splat(0.19862962962962963) * t100 * t102 + f64x8::splat(0.25189783950617284) * t100 * t109;
            let t114 = f64x8::splat(0.0);
            let t115 = t57 * t114;
            let t118 = t45 * t63;
            let t121 = t62 * t62;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t59 * t122;
            let t126 = -f64x8::splat(0.08222222222222222) * t92 * t118 - f64x8::splat(0.08222222222222222) * t123 * t93;
            let t128 = t66 * t114;
            let t131 = t112 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t93 + t126 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t128 * t93;
            let t136 = t4 * t18;
            let t138 = f64x8::splat(1.0) / t20 / t86;
            let t139 = t136 * t138;
            let t140 = t73 * t73;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t70 * t141;
            let t143 = t142 * t33;
            let t147 = ((t3).select(f64x8::splat(0.0), -t19 * t79 * t70 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t131 * t74 - f64x8::splat(0.0006934006726548522) * t139 * t143));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t147 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t150 = t27 * t36;
            let t153 = t51 * t53;
            let t154 = t45 * t48 * t153;
            let t155 = t150 * t40 * t154;
            let t158 = t100 * t150 * t153;
            let t160 = t99 * t48;
            let t161 = t97 * t160;
            let t162 = t106 * t53;
            let t164 = t161 * t162 * t150;
            let t166 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t155 + f64x8::splat(0.024828703703703704) * t158 - f64x8::splat(0.031487229938271605) * t164;
            let t168 = t115 * t27;
            let t170 = t36 * t40 * t45;
            let t171 = t168 * t170;
            let t173 = t40 * t45;
            let t174 = t173 * t63;
            let t175 = t150 * t174;
            let t177 = t123 * t27;
            let t178 = t177 * t170;
            let t180 = f64x8::splat(0.010277777777777778) * t175 + f64x8::splat(0.010277777777777778) * t178;
            let t182 = t128 * t27;
            let t183 = t182 * t170;
            let t185 = t166 * t60 + f64x8::splat(5.0) / f64x8::splat(72.0) * t171 + t180 * t68 - f64x8::splat(5.0) / f64x8::splat(72.0) * t183;
            let t192 = t136 / t20 / t34;
            let t193 = t142 * t27;
            let t197 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t185 * t74 + f64x8::splat(0.0002600252522455696) * t192 * t193));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t197;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t199 = t27 * t31;
            let t209 = f64x8::splat(5.0) / f64x8::splat(9.0) * t199 * t40 * t154 - f64x8::splat(0.19862962962962963) * t100 * t199 * t153 + f64x8::splat(0.25189783950617284) * t161 * t162 * t199;
            let t212 = t31 * t40 * t45;
            let t219 = -f64x8::splat(0.08222222222222222) * t199 * t174 - f64x8::splat(0.08222222222222222) * t177 * t212;
            let t223 = t209 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t168 * t212 + t219 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t182 * t212;
            let t228 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t223 * t74));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t228;
            acc_vtau = tvtau0;
            let t239 = t34 * t34;
            let t241 = f64x8::splat(1.0) / t20 / t239;
            let t242 = t136 * t241;
            let t248 = f64x8::splat(1.0) / t29 / t239;
            let t251 = f64x8::splat(40.0) / f64x8::splat(9.0) * t28 * t88 - f64x8::splat(11.0) / f64x8::splat(9.0) * t33 * t248;
            let t252 = t251 * t40;
            let t253 = t252 * t45;
            let t256 = t91 * t91;
            let t257 = t256 * t96;
            let t259 = t99 * t51 * t53;
            let t262 = t257 * t99;
            let t263 = t107 * t53;
            let t273 = t39 * t48;
            let t275 = f64x8::splat(1.0) / t105 / t50;
            let t276 = t275 * t53;
            let t277 = t276 * t256;
            let t280 = t53 * t251;
            let t281 = t107 * t280;
            let t284 = f64x8::splat(5.0) / f64x8::splat(9.0) * t253 * t54 - f64x8::splat(0.39725925925925926) * t257 * t259 + f64x8::splat(0.5037956790123457) * t262 * t263 - f64x8::splat(0.19862962962962963) * t100 * t251 * t51 * t53 - f64x8::splat(0.011094883230560388) * t39 * t256 * t162 + f64x8::splat(0.014070293140870518) * t273 * t277 + f64x8::splat(0.25189783950617284) * t100 * t281;
            let t286 = t112 * t114;
            let t289 = f64x8::splat(0.0);
            let t290 = t57 * t289;
            let t297 = t99 * t122;
            let t301 = f64x8::splat(1.0) / t121 / t62;
            let t302 = t59 * t301;
            let t307 = -f64x8::splat(0.08222222222222222) * t252 * t118 + f64x8::splat(0.09135802469135802) * t257 * t297 + f64x8::splat(0.09135802469135802) * t302 * t262 - f64x8::splat(0.08222222222222222) * t123 * t253;
            let t309 = t126 * t114;
            let t312 = t66 * t289;
            let t317 = t284 * t60 - f64x8::splat(10.0) / f64x8::splat(9.0) * t286 * t93 - f64x8::splat(25.0) / f64x8::splat(81.0) * t290 * t262 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t253 + t307 * t68 + f64x8::splat(10.0) / f64x8::splat(9.0) * t309 * t93 + f64x8::splat(25.0) / f64x8::splat(81.0) * t312 * t262 + f64x8::splat(5.0) / f64x8::splat(9.0) * t128 * t253;
            let t322 = t131 * t141;
            let t323 = t322 * t33;
            let t326 = t239 * t86;
            let t327 = f64x8::splat(1.0) / t326;
            let t328 = t136 * t327;
            let t330 = f64x8::splat(1.0) / t140 / t73;
            let t331 = t70 * t330;
            let t332 = v_sigma * v_sigma;
            let t333 = t332 * t21;
            let t334 = t331 * t333;
            let t338 = ((t3).select(f64x8::splat(0.0), t19 * t31 * t70 * t74 / f64x8::splat(12.0) - t19 * t79 * t131 * t74 / f64x8::splat(4.0) + f64x8::splat(0.0020802020179645567) * t242 * t143 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t317 * t74 - f64x8::splat(0.0013868013453097044) * t139 * t323 - f64x8::splat(7.511278503615601e-06) * t328 * t334));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t338 + f64x8::splat(4.0) * t147;
            acc_v2rho2 = tv2rho20;
            let t345 = t27 * t88;
            let t347 = t345 * t40 * t154;
            let t349 = t150 * t96;
            let t350 = t99 * t91;
            let t351 = t350 * t153;
            let t352 = t349 * t351;
            let t354 = t96 * t99;
            let t355 = t150 * t354;
            let t356 = t355 * t109;
            let t359 = t100 * t345 * t153;
            let t361 = t39 * t27;
            let t362 = t361 * t36;
            let t363 = t162 * t91;
            let t364 = t362 * t363;
            let t366 = t273 * t275;
            let t367 = t53 * t27;
            let t368 = t36 * t91;
            let t370 = t366 * t367 * t368;
            let t373 = t161 * t162 * t345;
            let t375 = f64x8::splat(5.0) / f64x8::splat(27.0) * t347 + f64x8::splat(0.04965740740740741) * t352 - f64x8::splat(0.06297445987654321) * t356 - f64x8::splat(0.06620987654320988) * t359 + f64x8::splat(0.0013868604038200485) * t364 - f64x8::splat(0.0017587866426088147) * t370 + f64x8::splat(0.08396594650205762) * t373;
            let t377 = t166 * t114;
            let t380 = t286 * t27;
            let t381 = t380 * t170;
            let t383 = t290 * t91;
            let t384 = t383 * t355;
            let t387 = t88 * t40 * t45;
            let t388 = t168 * t387;
            let t390 = t345 * t174;
            let t392 = t297 * t91;
            let t393 = t349 * t392;
            let t395 = t302 * t27;
            let t396 = t36 * t96;
            let t398 = t395 * t396 * t350;
            let t400 = t177 * t387;
            let t402 = -f64x8::splat(0.027407407407407408) * t390 - f64x8::splat(0.011419753086419753) * t393 - f64x8::splat(0.011419753086419753) * t398 - f64x8::splat(0.027407407407407408) * t400;
            let t404 = t180 * t114;
            let t407 = t309 * t27;
            let t408 = t407 * t170;
            let t410 = t312 * t91;
            let t411 = t410 * t355;
            let t413 = t182 * t387;
            let t415 = t375 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t377 * t93 + f64x8::splat(5.0) / f64x8::splat(72.0) * t381 + f64x8::splat(25.0) / f64x8::splat(648.0) * t384 - f64x8::splat(5.0) / f64x8::splat(27.0) * t388 + t402 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t404 * t93 - f64x8::splat(5.0) / f64x8::splat(72.0) * t408 - f64x8::splat(25.0) / f64x8::splat(648.0) * t411 + f64x8::splat(5.0) / f64x8::splat(27.0) * t413;
            let t420 = t185 * t141;
            let t421 = t420 * t33;
            let t426 = t322 * t27;
            let t429 = t239 * t34;
            let t430 = f64x8::splat(1.0) / t429;
            let t431 = t136 * t430;
            let t432 = t21 * v_sigma;
            let t433 = t331 * t432;
            let t437 = ((t3).select(f64x8::splat(0.0), -t19 * t79 * t185 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t415 * t74 - f64x8::splat(0.0006934006726548522) * t139 * t421 - f64x8::splat(0.0006067255885729957) * t139 * t193 + f64x8::splat(0.0002600252522455696) * t192 * t426 + f64x8::splat(2.8167294388558505e-06) * t431 * t433));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t437 + f64x8::splat(2.0) * t197;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t445 = t199 * t96;
            let t448 = t199 * t354;
            let t452 = t361 * t31;
            let t460 = -f64x8::splat(25.0) / f64x8::splat(27.0) * t155 - f64x8::splat(0.39725925925925926) * t445 * t351 + f64x8::splat(0.5037956790123457) * t448 * t109 + f64x8::splat(0.3310493827160494) * t158 - f64x8::splat(0.011094883230560388) * t452 * t363 + f64x8::splat(0.014070293140870518) * t366 * t367 * t31 * t91 - f64x8::splat(0.41982973251028805) * t164;
            let t462 = t209 * t114;
            let t473 = t31 * t96;
            let t478 = f64x8::splat(0.13703703703703704) * t175 + f64x8::splat(0.09135802469135802) * t445 * t392 + f64x8::splat(0.09135802469135802) * t395 * t473 * t350 + f64x8::splat(0.13703703703703704) * t178;
            let t480 = t219 * t114;
            let t488 = t460 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t462 * t93 - f64x8::splat(5.0) / f64x8::splat(9.0) * t380 * t212 - f64x8::splat(25.0) / f64x8::splat(81.0) * t383 * t448 + f64x8::splat(25.0) / f64x8::splat(27.0) * t171 + t478 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t480 * t93 + f64x8::splat(5.0) / f64x8::splat(9.0) * t407 * t212 + f64x8::splat(25.0) / f64x8::splat(81.0) * t410 * t448 - f64x8::splat(25.0) / f64x8::splat(27.0) * t183;
            let t493 = t223 * t141;
            let t494 = t493 * t33;
            let t498 = ((t3).select(f64x8::splat(0.0), -t19 * t79 * t223 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t488 * t74 - f64x8::splat(0.0006934006726548522) * t139 * t494));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t498 + f64x8::splat(2.0) * t228;
            acc_v2rhotau = tv2rhotau0;
            let t501 = t239 * v_rho;
            let t503 = f64x8::splat(1.0) / t20 / t501;
            let t504 = t21 * t503;
            let t505 = t504 * t96;
            let t506 = t505 * t259;
            let t508 = t160 * t162;
            let t509 = t505 * t508;
            let t511 = t39 * t21;
            let t514 = t511 * t503 * t106 * t53;
            let t516 = t53 * t21;
            let t518 = t366 * t516 * t503;
            let t520 = -f64x8::splat(0.012414351851851852) * t506 + f64x8::splat(0.015743614969135802) * t509 - f64x8::splat(0.00034671510095501213) * t514 + f64x8::splat(0.0004396966606522037) * t518;
            let t522 = t377 * t27;
            let t523 = t522 * t170;
            let t525 = t290 * t21;
            let t527 = t503 * t96 * t99;
            let t528 = t525 * t527;
            let t530 = t354 * t122;
            let t531 = t504 * t530;
            let t533 = t302 * t21;
            let t534 = t533 * t527;
            let t536 = f64x8::splat(0.002854938271604938) * t531 + f64x8::splat(0.002854938271604938) * t534;
            let t538 = t404 * t27;
            let t539 = t538 * t170;
            let t541 = t312 * t21;
            let t542 = t541 * t527;
            let t544 = t520 * t60 + f64x8::splat(5.0) / f64x8::splat(36.0) * t523 - f64x8::splat(25.0) / f64x8::splat(2592.0) * t528 + t536 * t68 - f64x8::splat(5.0) / f64x8::splat(36.0) * t539 + f64x8::splat(25.0) / f64x8::splat(2592.0) * t542;
            let t549 = t420 * t27;
            let t552 = f64x8::splat(1.0) / t501;
            let t553 = t136 * t552;
            let t554 = t331 * t21;
            let t558 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t544 * t74 + f64x8::splat(0.0005200505044911392) * t192 * t549 - f64x8::splat(1.0562735395709438e-06) * t553 * t554));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t558;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t560 = t21 * t241;
            let t561 = t560 * t96;
            let t562 = t561 * t259;
            let t564 = t561 * t508;
            let t568 = t511 * t241 * t106 * t53;
            let t571 = t366 * t516 * t241;
            let t573 = f64x8::splat(0.09931481481481481) * t562 - f64x8::splat(0.12594891975308642) * t564 + f64x8::splat(0.002773720807640097) * t568 - f64x8::splat(0.0035175732852176294) * t571;
            let t575 = t462 * t27;
            let t576 = t575 * t170;
            let t581 = t241 * t96 * t99;
            let t582 = t525 * t581;
            let t584 = t560 * t530;
            let t586 = t533 * t581;
            let t588 = -f64x8::splat(0.022839506172839506) * t584 - f64x8::splat(0.022839506172839506) * t586;
            let t590 = t480 * t27;
            let t591 = t590 * t170;
            let t595 = t541 * t581;
            let t597 = t573 * t60 + f64x8::splat(5.0) / f64x8::splat(72.0) * t576 - f64x8::splat(5.0) / f64x8::splat(9.0) * t522 * t212 + f64x8::splat(25.0) / f64x8::splat(324.0) * t582 + t588 * t68 - f64x8::splat(5.0) / f64x8::splat(72.0) * t591 + f64x8::splat(5.0) / f64x8::splat(9.0) * t538 * t212 - f64x8::splat(25.0) / f64x8::splat(324.0) * t595;
            let t602 = t493 * t27;
            let t606 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t597 * t74 + f64x8::splat(0.0002600252522455696) * t192 * t602));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t606;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t608 = t21 * t138;
            let t609 = t608 * t96;
            let t621 = -f64x8::splat(0.7945185185185185) * t609 * t259 + f64x8::splat(1.0075913580246914) * t609 * t508 - f64x8::splat(0.022189766461120777) * t511 * t138 * t106 * t53 + f64x8::splat(0.028140586281741035) * t366 * t516 * t138;
            let t626 = t138 * t96 * t99;
            let t633 = f64x8::splat(0.18271604938271604) * t608 * t530 + f64x8::splat(0.18271604938271604) * t533 * t626;
            let t639 = t621 * t60 - f64x8::splat(10.0) / f64x8::splat(9.0) * t575 * t212 - f64x8::splat(50.0) / f64x8::splat(81.0) * t525 * t626 + t633 * t68 + f64x8::splat(10.0) / f64x8::splat(9.0) * t590 * t212 + f64x8::splat(50.0) / f64x8::splat(81.0) * t541 * t626;
            let t644 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t639 * t74));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t644;
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
