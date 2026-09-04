//! GGA_X_SFAT fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`
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
pub fn gga_x_sfat_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t17 / t4 * t3;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t24;
            let t27 = t24 * t20;
            let t28 = t25 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = t30 * v_sigma;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t36 = ((v_sigma).sqrt());
            let t37 = t29 * t36;
            let t39 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = (simd::ln(t39 * t37 + ((((t39 * t37) * (t39 * t37)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t41 * t39;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t42 * t37;
            let t46 = f64x8::splat(1.0) / t45;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t46 * t35 * t31 * t28;
            let t54 = f64x8::splat(1.0) / t51 * t26 * t20 * f64x8::splat(M_PI);
            let t55 = ((t54).sqrt());
            let t57 = f64x8::splat(1.0) / t55 * param_hyb_omega_0;
            let t58 = v_rho * t11;
            let t59 = (simd::cbrt(t58));
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t60 * t29;
            let t63 = t61 * t57 / f64x8::splat(2.0);
            let t64 = (f64x8::splat(1.92)).simd_le(t63);
            let t65 = (f64x8::splat(1.92)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(1.92)));
            let t67 = t66 * t66;
            let t68 = t67 * t67;
            let t69 = t68 * t68;
            let t70 = t69 * t69;
            let t71 = t70 * t70;
            let t73 = f64x8::splat(1.0) / t71 / t67;
            let t76 = f64x8::splat(1.0) / t71 / t68;
            let t78 = f64x8::splat(1.0) / t68;
            let t80 = t68 * t67;
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = f64x8::splat(1.0) / t69;
            let t85 = t69 * t67;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t69 * t68;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t69 * t80;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = f64x8::splat(1.0) / t70;
            let t97 = f64x8::splat(1.0) / t70 / t67;
            let t100 = f64x8::splat(1.0) / t70 / t68;
            let t103 = f64x8::splat(1.0) / t70 / t80;
            let t106 = f64x8::splat(1.0) / t70 / t69;
            let t109 = f64x8::splat(1.0) / t70 / t85;
            let t112 = f64x8::splat(1.0) / t70 / t88;
            let t115 = f64x8::splat(1.0) / t70 / t91;
            let t117 = f64x8::splat(1.0) / t71;
            let t121 = t73 / f64x8::splat(5985.0) - t76 / f64x8::splat(7030.0) - t78 / f64x8::splat(30.0) + t81 / f64x8::splat(70.0) - t83 / f64x8::splat(135.0) + t86 / f64x8::splat(231.0) - t89 / f64x8::splat(364.0) + t92 / f64x8::splat(540.0) - t94 / f64x8::splat(765.0) + t97 / f64x8::splat(1045.0) - t100 / f64x8::splat(1386.0) + t103 / f64x8::splat(1794.0) - t106 / f64x8::splat(2275.0) + t109 / f64x8::splat(2835.0) - t112 / f64x8::splat(3480.0) + t115 / f64x8::splat(4216.0) - t117 / f64x8::splat(5049.0) + f64x8::splat(1.0) / t67 / f64x8::splat(9.0);
            let t122 = ((t65).select(f64x8::splat(1.92), t63));
            let t123 = (simd::atan2(f64x8::splat(1.0), t122));
            let t124 = t122 * t122;
            let t125 = t124 + f64x8::splat(3.0);
            let t126 = f64x8::splat(1.0) / t124;
            let t127 = f64x8::splat(1.0) + t126;
            let t128 = (simd::ln(t127));
            let t130 = -t125 * t128 + f64x8::splat(1.0);
            let t133 = t123 + t130 * t122 / f64x8::splat(4.0);
            let t137 = ((t64).select(t121, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t122));
            let t138 = t137 * t19;
            let t142 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t138 * t18));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t33;
            let t144 = t137 * t143;
            let t148 = t67 * t66;
            let t150 = f64x8::splat(1.0) / t71 / t148;
            let t153 = f64x8::splat(1.0) / t55 / t54 * param_hyb_omega_0;
            let t155 = f64x8::splat(M_PI) * t61 * t153;
            let t156 = t51 * t51;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t157 * t25;
            let t159 = t32 * v_rho;
            let t161 = f64x8::splat(1.0) / t33 / t159;
            let t166 = v_sigma * t25;
            let t167 = t166 * t27;
            let t168 = t35 * t30;
            let t169 = t45 * t45;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = t41 / t19 / t32;
            let t177 = t31 * t35 + f64x8::splat(1.0);
            let t178 = ((t177).sqrt());
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t179 * t161;
            let t183 = -f64x8::splat(0.0336) * t173 * t37 - f64x8::splat(0.0336) * t180 * t31;
            let t184 = t183 * t170;
            let t185 = t184 * t168;
            let t188 = -f64x8::splat(0.002488888888888889) * t46 * t161 * t31 * t28 - f64x8::splat(0.0009333333333333333) * t185 * t167;
            let t194 = f64x8::splat(1.0) / t59 / t58;
            let t195 = t194 * t29;
            let t199 = t188 * t158 * t27 * t155 / f64x8::splat(4.0) - t11 * t195 * t57 / f64x8::splat(6.0);
            let t200 = ((t65).select(t199, f64x8::splat(0.0)));
            let t203 = t68 * t66;
            let t205 = f64x8::splat(1.0) / t71 / t203;
            let t208 = f64x8::splat(1.0) / t203;
            let t211 = t68 * t148;
            let t212 = f64x8::splat(1.0) / t211;
            let t215 = t69 * t66;
            let t216 = f64x8::splat(1.0) / t215;
            let t219 = t69 * t148;
            let t220 = f64x8::splat(1.0) / t219;
            let t223 = t69 * t203;
            let t224 = f64x8::splat(1.0) / t223;
            let t227 = t69 * t211;
            let t228 = f64x8::splat(1.0) / t227;
            let t232 = f64x8::splat(1.0) / t70 / t66;
            let t236 = f64x8::splat(1.0) / t70 / t148;
            let t240 = f64x8::splat(1.0) / t70 / t203;
            let t244 = f64x8::splat(1.0) / t70 / t211;
            let t248 = f64x8::splat(1.0) / t70 / t215;
            let t252 = f64x8::splat(1.0) / t70 / t219;
            let t256 = f64x8::splat(1.0) / t70 / t223;
            let t260 = f64x8::splat(1.0) / t70 / t227;
            let t264 = f64x8::splat(1.0) / t71 / t66;
            let t267 = f64x8::splat(1.0) / t148;
            let t270 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t200 * t150 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t200 * t205 + f64x8::splat(2.0) / f64x8::splat(15.0) * t200 * t208 - f64x8::splat(3.0) / f64x8::splat(35.0) * t200 * t212 + f64x8::splat(8.0) / f64x8::splat(135.0) * t200 * t216 - f64x8::splat(10.0) / f64x8::splat(231.0) * t200 * t220 + f64x8::splat(3.0) / f64x8::splat(91.0) * t200 * t224 - f64x8::splat(7.0) / f64x8::splat(270.0) * t200 * t228 + f64x8::splat(16.0) / f64x8::splat(765.0) * t200 * t232 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t200 * t236 + f64x8::splat(10.0) / f64x8::splat(693.0) * t200 * t240 - f64x8::splat(11.0) / f64x8::splat(897.0) * t200 * t244 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t200 * t248 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t200 * t252 + f64x8::splat(7.0) / f64x8::splat(870.0) * t200 * t256 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t200 * t260 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t200 * t264 - f64x8::splat(2.0) / f64x8::splat(9.0) * t200 * t267;
            let t271 = ((t65).select(f64x8::splat(0.0), t199));
            let t274 = f64x8::splat(1.0) / t127;
            let t280 = t124 * t122;
            let t281 = f64x8::splat(1.0) / t280;
            let t282 = t281 * t125;
            let t283 = t274 * t271;
            let t286 = -f64x8::splat(2.0) * t122 * t128 * t271 + f64x8::splat(2.0) * t282 * t283;
            let t289 = -t274 * t126 * t271 + t130 * t271 / f64x8::splat(4.0) + t286 * t122 / f64x8::splat(4.0);
            let t293 = ((t64).select(t270, -f64x8::splat(8.0) / f64x8::splat(3.0) * t289 * t122 - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t271));
            let t294 = t293 * t19;
            let t302 = ((t2).select(f64x8::splat(0.0), -t51 * t144 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t294 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t188 * t138 * t18));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t302 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t309 = t29 / t36;
            let t314 = f64x8::splat(0.0126) * t42 * t309 + f64x8::splat(0.0126) * t179 * t168;
            let t315 = t314 * t170;
            let t316 = t315 * t168;
            let t319 = f64x8::splat(0.0009333333333333333) * t46 * t168 * t28 - f64x8::splat(0.0009333333333333333) * t316 * t167;
            let t323 = t319 * t158 * t27 * t155 / f64x8::splat(4.0);
            let t324 = ((t65).select(t323, f64x8::splat(0.0)));
            let t325 = t324 * t150;
            let t327 = t324 * t205;
            let t329 = t324 * t208;
            let t331 = t324 * t212;
            let t333 = t324 * t216;
            let t335 = t324 * t220;
            let t337 = t324 * t224;
            let t339 = t324 * t228;
            let t341 = t324 * t232;
            let t343 = t324 * t236;
            let t345 = t324 * t240;
            let t347 = t324 * t244;
            let t349 = t324 * t248;
            let t351 = t324 * t252;
            let t353 = t324 * t256;
            let t355 = t324 * t260;
            let t357 = t324 * t264;
            let t361 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t325 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t327 + f64x8::splat(2.0) / f64x8::splat(15.0) * t329 - f64x8::splat(3.0) / f64x8::splat(35.0) * t331 + f64x8::splat(8.0) / f64x8::splat(135.0) * t333 - f64x8::splat(10.0) / f64x8::splat(231.0) * t335 + f64x8::splat(3.0) / f64x8::splat(91.0) * t337 - f64x8::splat(7.0) / f64x8::splat(270.0) * t339 + f64x8::splat(16.0) / f64x8::splat(765.0) * t341 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t343 + f64x8::splat(10.0) / f64x8::splat(693.0) * t345 - f64x8::splat(11.0) / f64x8::splat(897.0) * t347 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t349 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t351 + f64x8::splat(7.0) / f64x8::splat(870.0) * t353 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t355 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t357 - f64x8::splat(2.0) / f64x8::splat(9.0) * t324 * t267;
            let t362 = ((t65).select(f64x8::splat(0.0), t323));
            let t364 = t126 * t362;
            let t370 = t274 * t362;
            let t373 = -f64x8::splat(2.0) * t122 * t128 * t362 + f64x8::splat(2.0) * t282 * t370;
            let t376 = -t274 * t364 + t130 * t362 / f64x8::splat(4.0) + t373 * t122 / f64x8::splat(4.0);
            let t380 = ((t64).select(t361, -f64x8::splat(8.0) / f64x8::splat(3.0) * t376 * t122 - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t362));
            let t381 = t380 * t19;
            let t388 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t319 * t138 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t381 * t18));
            let tvsigma0 = f64x8::splat(2.0) * t388 * v_rho;
            acc_vsigma = tvsigma0;
            let t392 = f64x8::splat(1.0) / t33 / v_rho;
            let t393 = t137 * t392;
            let t397 = t293 * t143;
            let t404 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t406 = t23 * t23;
            let t407 = f64x8::splat(1.0) / t406;
            let t408 = t25 * t25;
            let t409 = t408 * t407;
            let t415 = f64x8::splat(1.0) / t55 / t157 / t409 / t3 / t404 * param_hyb_omega_0 / f64x8::splat(3.0);
            let t417 = t404 * t61 * t415;
            let t418 = t407 * t3;
            let t419 = t156 * t156;
            let t420 = f64x8::splat(1.0) / t419;
            let t421 = t420 * t408;
            let t422 = t188 * t188;
            let t428 = f64x8::splat(M_PI) * t195 * t153;
            let t429 = t188 * t157;
            let t434 = t156 * t51;
            let t435 = f64x8::splat(1.0) / t434;
            let t436 = t435 * t25;
            let t441 = t32 * t32;
            let t443 = f64x8::splat(1.0) / t33 / t441;
            let t448 = t161 * t30;
            let t449 = t184 * t448;
            let t453 = f64x8::splat(1.0) / t169 / t45;
            let t454 = t183 * t183;
            let t455 = t454 * t453;
            let t456 = t455 * t168;
            let t461 = t41 / t19 / t159;
            let t464 = t179 * t443;
            let t467 = v_sigma * v_sigma;
            let t468 = t29 * t467;
            let t471 = f64x8::splat(1.0) / t19 / t441 / t159;
            let t473 = f64x8::splat(1.0) / t178 / t177;
            let t477 = f64x8::splat(0.0784) * t461 * t37 + f64x8::splat(0.168) * t464 * t31 - f64x8::splat(0.0896) * t473 * t471 * t468;
            let t478 = t477 * t170;
            let t479 = t478 * t168;
            let t482 = f64x8::splat(0.009125925925925926) * t46 * t443 * t31 * t28 + f64x8::splat(0.004977777777777778) * t449 * t167 + f64x8::splat(0.0018666666666666666) * t456 * t167 - f64x8::splat(0.0009333333333333333) * t479 * t167;
            let t487 = t11 * t11;
            let t490 = f64x8::splat(1.0) / t59 / t32 / t487;
            let t491 = t490 * t29;
            let t495 = f64x8::splat(9.0) / f64x8::splat(8.0) * t422 * t421 * t418 * t417 - t11 * t429 * t28 * t428 / f64x8::splat(6.0) - t422 * t436 * t27 * t155 / f64x8::splat(2.0) + t482 * t158 * t27 * t155 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t487 * t491 * t57;
            let t496 = ((t65).select(t495, f64x8::splat(0.0)));
            let t499 = t496 * t264;
            let t501 = t496 * t260;
            let t503 = t496 * t256;
            let t505 = t496 * t252;
            let t507 = t496 * t248;
            let t509 = t496 * t244;
            let t511 = t496 * t240;
            let t513 = t496 * t236;
            let t515 = t496 * t232;
            let t517 = t496 * t228;
            let t519 = t496 * t224;
            let t521 = t496 * t220;
            let t523 = t496 * t216;
            let t525 = t496 * t212;
            let t527 = t496 * t208;
            let t529 = t496 * t205;
            let t531 = t496 * t150;
            let t533 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t496 * t267 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t499 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t501 + f64x8::splat(7.0) / f64x8::splat(870.0) * t503 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t505 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t507 - f64x8::splat(11.0) / f64x8::splat(897.0) * t509 + f64x8::splat(10.0) / f64x8::splat(693.0) * t511 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t513 + f64x8::splat(16.0) / f64x8::splat(765.0) * t515 - f64x8::splat(7.0) / f64x8::splat(270.0) * t517 + f64x8::splat(3.0) / f64x8::splat(91.0) * t519 - f64x8::splat(10.0) / f64x8::splat(231.0) * t521 + f64x8::splat(8.0) / f64x8::splat(135.0) * t523 - f64x8::splat(3.0) / f64x8::splat(35.0) * t525 + f64x8::splat(2.0) / f64x8::splat(15.0) * t527 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t529 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t531;
            let t534 = t200 * t200;
            let t566 = f64x8::splat(1.0) / t71 / t80;
            let t573 = -f64x8::splat(32.0) / f64x8::splat(153.0) * t534 * t73 + f64x8::splat(2.0) / f64x8::splat(3.0) * t534 * t78 + f64x8::splat(26.0) / f64x8::splat(105.0) * t534 * t112 - f64x8::splat(7.0) / f64x8::splat(30.0) * t534 * t115 + f64x8::splat(15.0) / f64x8::splat(68.0) * t534 * t117 + f64x8::splat(11.0) / f64x8::splat(39.0) * t534 * t106 - f64x8::splat(24.0) / f64x8::splat(91.0) * t534 * t109 - f64x8::splat(16.0) / f64x8::splat(45.0) * t534 * t97 + f64x8::splat(18.0) / f64x8::splat(55.0) * t534 * t100 - f64x8::splat(10.0) / f64x8::splat(33.0) * t534 * t103 - f64x8::splat(3.0) / f64x8::splat(7.0) * t534 * t92 + f64x8::splat(7.0) / f64x8::splat(18.0) * t534 * t94 + f64x8::splat(3.0) / f64x8::splat(5.0) * t534 * t83 - f64x8::splat(8.0) / f64x8::splat(15.0) * t534 * t86 + f64x8::splat(10.0) / f64x8::splat(21.0) * t534 * t89 - f64x8::splat(18.0) / f64x8::splat(95.0) * t534 * t566 - f64x8::splat(2.0) / f64x8::splat(3.0) * t534 * t81 + f64x8::splat(34.0) / f64x8::splat(171.0) * t534 * t76;
            let t575 = ((t65).select(f64x8::splat(0.0), t495));
            let t580 = t126 * t575;
            let t582 = t271 * t271;
            let t583 = t281 * t582;
            let t586 = t124 * t124;
            let t588 = f64x8::splat(1.0) / t586 / t122;
            let t590 = t127 * t127;
            let t591 = f64x8::splat(1.0) / t590;
            let t606 = f64x8::splat(1.0) / t586;
            let t607 = t606 * t125;
            let t608 = t274 * t582;
            let t611 = t274 * t575;
            let t615 = f64x8::splat(1.0) / t586 / t124;
            let t616 = t615 * t125;
            let t617 = t591 * t582;
            let t620 = -f64x8::splat(2.0) * t122 * t128 * t575 + f64x8::splat(8.0) * t126 * t274 * t582 - f64x8::splat(2.0) * t128 * t582 + f64x8::splat(2.0) * t282 * t611 - f64x8::splat(6.0) * t607 * t608 + f64x8::splat(4.0) * t616 * t617;
            let t623 = -t274 * t580 + f64x8::splat(2.0) * t274 * t583 - f64x8::splat(2.0) * t591 * t588 * t582 + t130 * t575 / f64x8::splat(4.0) + t286 * t271 / f64x8::splat(2.0) + t620 * t122 / f64x8::splat(4.0);
            let t627 = ((t64).select(t533 + t573, -f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t575 - f64x8::splat(16.0) / f64x8::splat(3.0) * t289 * t271 - f64x8::splat(8.0) / f64x8::splat(3.0) * t623 * t122));
            let t628 = t627 * t19;
            let t639 = ((t2).select(f64x8::splat(0.0), t51 * t393 * t18 / f64x8::splat(12.0) - t51 * t397 * t18 / f64x8::splat(4.0) - t188 * t144 * t18 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t628 * t18 - f64x8::splat(3.0) / f64x8::splat(4.0) * t188 * t294 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t482 * t138 * t18));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t639 + f64x8::splat(4.0) * t302;
            acc_v2rho2 = tv2rho20;
            let t642 = t380 * t143;
            let t646 = t324 * t117;
            let t649 = t324 * t73;
            let t652 = t324 * t78;
            let t655 = t324 * t109;
            let t658 = t324 * t112;
            let t661 = t324 * t115;
            let t664 = t324 * t97;
            let t667 = t324 * t100;
            let t670 = t324 * t103;
            let t673 = t324 * t106;
            let t676 = t324 * t89;
            let t679 = t324 * t92;
            let t682 = t324 * t94;
            let t685 = t324 * t81;
            let t688 = t324 * t83;
            let t691 = t324 * t86;
            let t694 = t324 * t76;
            let t697 = t324 * t566;
            let t700 = f64x8::splat(15.0) / f64x8::splat(68.0) * t200 * t646 - f64x8::splat(32.0) / f64x8::splat(153.0) * t200 * t649 + f64x8::splat(2.0) / f64x8::splat(3.0) * t200 * t652 - f64x8::splat(24.0) / f64x8::splat(91.0) * t200 * t655 + f64x8::splat(26.0) / f64x8::splat(105.0) * t200 * t658 - f64x8::splat(7.0) / f64x8::splat(30.0) * t200 * t661 - f64x8::splat(16.0) / f64x8::splat(45.0) * t200 * t664 + f64x8::splat(18.0) / f64x8::splat(55.0) * t200 * t667 - f64x8::splat(10.0) / f64x8::splat(33.0) * t200 * t670 + f64x8::splat(11.0) / f64x8::splat(39.0) * t200 * t673 + f64x8::splat(10.0) / f64x8::splat(21.0) * t200 * t676 - f64x8::splat(3.0) / f64x8::splat(7.0) * t200 * t679 + f64x8::splat(7.0) / f64x8::splat(18.0) * t200 * t682 - f64x8::splat(2.0) / f64x8::splat(3.0) * t200 * t685 + f64x8::splat(3.0) / f64x8::splat(5.0) * t200 * t688 - f64x8::splat(8.0) / f64x8::splat(15.0) * t200 * t691 + f64x8::splat(34.0) / f64x8::splat(171.0) * t200 * t694 - f64x8::splat(18.0) / f64x8::splat(95.0) * t200 * t697;
            let t701 = t408 * t418;
            let t702 = t319 * t420;
            let t707 = t319 * t157;
            let t712 = t319 * t435;
            let t722 = t315 * t448;
            let t725 = t314 * t453;
            let t726 = t183 * t725;
            let t734 = t441 * t32;
            let t736 = f64x8::splat(1.0) / t19 / t734;
            let t737 = t736 * t29;
            let t738 = v_sigma * t473;
            let t741 = -f64x8::splat(0.0168) * t173 * t309 - f64x8::splat(0.0504) * t179 * t448 + f64x8::splat(0.0336) * t738 * t737;
            let t742 = t741 * t170;
            let t743 = t742 * t168;
            let t746 = -f64x8::splat(0.002488888888888889) * t46 * t448 * t28 - f64x8::splat(0.0009333333333333333) * t185 * t28 + f64x8::splat(0.002488888888888889) * t722 * t167 + f64x8::splat(0.0018666666666666666) * t726 * t168 * t167 - f64x8::splat(0.0009333333333333333) * t743 * t167;
            let t751 = f64x8::splat(9.0) / f64x8::splat(8.0) * t188 * t702 * t701 * t417 - t11 * t707 * t28 * t428 / f64x8::splat(12.0) - t188 * t712 * t28 * t155 / f64x8::splat(2.0) + t746 * t158 * t27 * t155 / f64x8::splat(4.0);
            let t752 = ((t65).select(t751, f64x8::splat(0.0)));
            let t755 = t752 * t264;
            let t757 = t752 * t260;
            let t759 = t752 * t256;
            let t761 = t752 * t252;
            let t763 = t752 * t248;
            let t765 = t752 * t244;
            let t767 = t752 * t240;
            let t769 = t752 * t236;
            let t771 = t752 * t232;
            let t773 = t752 * t228;
            let t775 = t752 * t224;
            let t777 = t752 * t220;
            let t779 = t752 * t216;
            let t781 = t752 * t212;
            let t783 = t752 * t208;
            let t785 = t752 * t205;
            let t787 = t752 * t150;
            let t789 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t752 * t267 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t755 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t757 + f64x8::splat(7.0) / f64x8::splat(870.0) * t759 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t761 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t763 - f64x8::splat(11.0) / f64x8::splat(897.0) * t765 + f64x8::splat(10.0) / f64x8::splat(693.0) * t767 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t769 + f64x8::splat(16.0) / f64x8::splat(765.0) * t771 - f64x8::splat(7.0) / f64x8::splat(270.0) * t773 + f64x8::splat(3.0) / f64x8::splat(91.0) * t775 - f64x8::splat(10.0) / f64x8::splat(231.0) * t777 + f64x8::splat(8.0) / f64x8::splat(135.0) * t779 - f64x8::splat(3.0) / f64x8::splat(35.0) * t781 + f64x8::splat(2.0) / f64x8::splat(15.0) * t783 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t785 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t787;
            let t791 = ((t65).select(f64x8::splat(0.0), t751));
            let t795 = t126 * t791;
            let t797 = t281 * t362;
            let t800 = t588 * t362;
            let t801 = t271 * t591;
            let t818 = t271 * t370;
            let t821 = t274 * t791;
            let t824 = t591 * t362;
            let t825 = t271 * t824;
            let t828 = -f64x8::splat(2.0) * t122 * t128 * t791 - f64x8::splat(2.0) * t128 * t271 * t362 + f64x8::splat(2.0) * t282 * t821 + f64x8::splat(8.0) * t283 * t364 - f64x8::splat(6.0) * t607 * t818 + f64x8::splat(4.0) * t616 * t825;
            let t831 = -t274 * t795 + f64x8::splat(2.0) * t283 * t797 - f64x8::splat(2.0) * t801 * t800 + t130 * t791 / f64x8::splat(4.0) + t286 * t362 / f64x8::splat(4.0) + t373 * t271 / f64x8::splat(4.0) + t828 * t122 / f64x8::splat(4.0);
            let t835 = ((t64).select(t700 + t789, -f64x8::splat(8.0) / f64x8::splat(3.0) * t831 * t122 - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t791 - f64x8::splat(8.0) / f64x8::splat(3.0) * t376 * t271 - f64x8::splat(8.0) / f64x8::splat(3.0) * t289 * t362));
            let t836 = t835 * t19;
            let t853 = ((t2).select(f64x8::splat(0.0), -t51 * t642 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t836 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t188 * t381 * t18 - t319 * t144 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t319 * t294 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t746 * t138 * t18));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t853 + f64x8::splat(2.0) * t388;
            acc_v2rhosigma = tv2rhosigma0;
            let t856 = t319 * t319;
            let t867 = t314 * t314;
            let t868 = t867 * t453;
            let t869 = t868 * t168;
            let t874 = t29 / t36 / v_sigma;
            let t877 = f64x8::splat(1.0) / v_sigma;
            let t878 = t30 * t877;
            let t879 = t179 * t35;
            let t882 = t441 * v_rho;
            let t884 = f64x8::splat(1.0) / t19 / t882;
            let t888 = -f64x8::splat(0.0063) * t42 * t874 + f64x8::splat(0.0063) * t879 * t878 - f64x8::splat(0.0126) * t473 * t884 * t29;
            let t889 = t888 * t170;
            let t890 = t889 * t168;
            let t893 = -f64x8::splat(0.0018666666666666666) * t316 * t28 + f64x8::splat(0.0018666666666666666) * t869 * t167 - f64x8::splat(0.0009333333333333333) * t890 * t167;
            let t898 = f64x8::splat(9.0) / f64x8::splat(8.0) * t856 * t421 * t418 * t417 - t856 * t436 * t27 * t155 / f64x8::splat(2.0) + t893 * t158 * t27 * t155 / f64x8::splat(4.0);
            let t899 = ((t65).select(t898, f64x8::splat(0.0)));
            let t902 = t899 * t264;
            let t904 = t899 * t260;
            let t906 = t899 * t256;
            let t908 = t899 * t252;
            let t910 = t899 * t248;
            let t912 = t899 * t244;
            let t914 = t899 * t240;
            let t916 = t899 * t236;
            let t918 = t899 * t232;
            let t920 = t899 * t228;
            let t922 = t899 * t224;
            let t924 = t899 * t220;
            let t926 = t899 * t216;
            let t928 = t324 * t324;
            let t929 = t928 * t117;
            let t931 = t928 * t73;
            let t935 = t928 * t109;
            let t937 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t899 * t267 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t902 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t904 + f64x8::splat(7.0) / f64x8::splat(870.0) * t906 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t908 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t910 - f64x8::splat(11.0) / f64x8::splat(897.0) * t912 + f64x8::splat(10.0) / f64x8::splat(693.0) * t914 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t916 + f64x8::splat(16.0) / f64x8::splat(765.0) * t918 - f64x8::splat(7.0) / f64x8::splat(270.0) * t920 + f64x8::splat(3.0) / f64x8::splat(91.0) * t922 - f64x8::splat(10.0) / f64x8::splat(231.0) * t924 + f64x8::splat(8.0) / f64x8::splat(135.0) * t926 + f64x8::splat(15.0) / f64x8::splat(68.0) * t929 - f64x8::splat(32.0) / f64x8::splat(153.0) * t931 + f64x8::splat(2.0) / f64x8::splat(3.0) * t928 * t78 - f64x8::splat(24.0) / f64x8::splat(91.0) * t935;
            let t938 = t928 * t112;
            let t940 = t928 * t115;
            let t942 = t928 * t100;
            let t944 = t928 * t103;
            let t946 = t928 * t106;
            let t948 = t928 * t92;
            let t950 = t928 * t94;
            let t952 = t928 * t97;
            let t954 = t928 * t83;
            let t956 = t928 * t86;
            let t958 = t928 * t89;
            let t960 = t928 * t76;
            let t962 = t928 * t566;
            let t964 = t928 * t81;
            let t966 = t899 * t212;
            let t968 = t899 * t208;
            let t970 = t899 * t205;
            let t972 = t899 * t150;
            let t974 = f64x8::splat(26.0) / f64x8::splat(105.0) * t938 - f64x8::splat(7.0) / f64x8::splat(30.0) * t940 + f64x8::splat(18.0) / f64x8::splat(55.0) * t942 - f64x8::splat(10.0) / f64x8::splat(33.0) * t944 + f64x8::splat(11.0) / f64x8::splat(39.0) * t946 - f64x8::splat(3.0) / f64x8::splat(7.0) * t948 + f64x8::splat(7.0) / f64x8::splat(18.0) * t950 - f64x8::splat(16.0) / f64x8::splat(45.0) * t952 + f64x8::splat(3.0) / f64x8::splat(5.0) * t954 - f64x8::splat(8.0) / f64x8::splat(15.0) * t956 + f64x8::splat(10.0) / f64x8::splat(21.0) * t958 + f64x8::splat(34.0) / f64x8::splat(171.0) * t960 - f64x8::splat(18.0) / f64x8::splat(95.0) * t962 - f64x8::splat(2.0) / f64x8::splat(3.0) * t964 - f64x8::splat(3.0) / f64x8::splat(35.0) * t966 + f64x8::splat(2.0) / f64x8::splat(15.0) * t968 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t970 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t972;
            let t976 = ((t65).select(f64x8::splat(0.0), t898));
            let t981 = t126 * t976;
            let t983 = t362 * t362;
            let t984 = t281 * t983;
            let t987 = t588 * t983;
            let t1002 = t274 * t983;
            let t1005 = t274 * t976;
            let t1008 = t591 * t983;
            let t1011 = -f64x8::splat(2.0) * t122 * t128 * t976 + f64x8::splat(8.0) * t126 * t274 * t983 - f64x8::splat(6.0) * t1002 * t607 + f64x8::splat(2.0) * t1005 * t282 + f64x8::splat(4.0) * t1008 * t616 - f64x8::splat(2.0) * t128 * t983;
            let t1014 = -t274 * t981 + f64x8::splat(2.0) * t274 * t984 - f64x8::splat(2.0) * t591 * t987 + t130 * t976 / f64x8::splat(4.0) + t373 * t362 / f64x8::splat(2.0) + t1011 * t122 / f64x8::splat(4.0);
            let t1018 = ((t64).select(t937 + t974, -f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t976 - f64x8::splat(16.0) / f64x8::splat(3.0) * t376 * t362 - f64x8::splat(8.0) / f64x8::splat(3.0) * t1014 * t122));
            let t1019 = t1018 * t19;
            let t1030 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t1019 * t18 - f64x8::splat(3.0) / f64x8::splat(4.0) * t319 * t381 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t893 * t138 * t18));
            let tv2sigma20 = f64x8::splat(2.0) * t1030 * v_rho;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
