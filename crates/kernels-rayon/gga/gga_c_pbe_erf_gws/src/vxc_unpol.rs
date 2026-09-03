//! GGA_C_PBE_ERF_GWS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_erf_gws.c`
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
pub fn gga_c_pbe_erf_gws_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_a_c: f64,
    param_beta: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_a_c = f64x8::splat(param_a_c);
    let param_beta = f64x8::splat(param_beta);
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = t5 * t22;
            let t24 = t20 * t23;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t45 * t54;
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = t58 - f64x8::splat(1.0);
            let t60 = f64x8::splat(2.0) * t59;
            let t62 = f64x8::splat(2.923025) * param_hyb_omega_0 * t13;
            let t64 = (simd::cbrt(f64x8::splat(9.0)));
            let t65 = t64 * t64;
            let t73 = param_hyb_omega_0 * param_hyb_omega_0;
            let t75 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t5 * t65 * t3 / t59 / f64x8::splat(12.0)) * t73 * t1;
            let t76 = t3 * t6;
            let t77 = t76 * t8;
            let t80 = t73 * param_hyb_omega_0;
            let t81 = t13 * t10;
            let t84 = f64x8::splat(1.0) + t62 + t75 * t77 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t80 * t81;
            let t85 = t73 * t1;
            let t88 = f64x8::splat(1.0) + t62 + f64x8::splat(0.8621275) * t85 * t77;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = (simd::ln(t84 * t89));
            let t93 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t94 = f64x8::splat(1.0) / t93;
            let t96 = f64x8::splat(1.0) / v_rho;
            let t100 = t3 * t2;
            let t101 = t1 * t100;
            let t103 = f64x8::splat(1.0) / t7 / v_rho;
            let t104 = t6 * t103;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.005175) * t10 + f64x8::splat(0.0204825) * t24 - f64x8::splat(0.0030486129349252553) * t96 + f64x8::splat(0.0003485625) * t101 * t104;
            let t110 = (simd::exp(-f64x8::splat(0.1881) * t10));
            let t111 = f64x8::splat(M_SQRT2);
            let t112 = t110 * t111;
            let t116 = t18 * t19 * t94;
            let t117 = t116 * t5;
            let t119 = f64x8::splat(1.0) / t21 / v_rho;
            let t121 = t4 * t9 * t39;
            let t124 = (f64x8::splat(1.0) - f64x8::splat(0.0056675) * t121) * t65;
            let t125 = f64x8::splat(1.0) / t100;
            let t126 = t124 * t125;
            let t127 = t1 * t21;
            let t129 = t39 * t39;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t121 + f64x8::splat(0.01) * t20 * t23 * t129;
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = t126 * t127 * t134 / f64x8::splat(15.0);
            let t139 = (simd::exp(-f64x8::splat(0.0775) * t10));
            let t142 = -f64x8::splat(1.2375) * t10 + t24 / f64x8::splat(4.0);
            let t143 = t139 * t142;
            let t144 = f64x8::splat(M_PI) * v_rho;
            let t147 = t137 + f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * t144;
            let t154 = t107 * t110;
            let t156 = t154 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t159 = t5 * t119;
            let t161 = (simd::exp(-f64x8::splat(0.13675) * t10));
            let t164 = -f64x8::splat(0.097) * t10 + f64x8::splat(0.169) * t24;
            let t166 = t161 * t164 * t1;
            let t167 = f64x8::splat(1.0) / t19;
            let t168 = t167 * t6;
            let t169 = t168 * t21;
            let t172 = t65 * t125;
            let t175 = t137 + t166 * t169 / f64x8::splat(3.0) - t172 * t127 / f64x8::splat(15.0);
            let t179 = -t32 + t57;
            let t184 = t73 * t73;
            let t186 = t116 * t159;
            let t187 = t184 * param_hyb_omega_0;
            let t188 = t111 * t187;
            let t189 = t154 * t188;
            let t195 = v_rho * v_rho;
            let t196 = f64x8::splat(1.0) / t195;
            let t200 = t184 * t73;
            let t203 = f64x8::splat(1.0) / t21 / t195;
            let t205 = t184 * t184;
            let t209 = t60 * t91 * t94 + (-f64x8::splat(0.031505407223141116) * t96 * t107 * t112 - f64x8::splat(0.005388405304614574) * t117 * t119 * t147 * t111) * t80 + (-f64x8::splat(0.0837628205355044) * t96 * t156 - f64x8::splat(0.011938374665504766) * t116 * t159 * t175 + f64x8::splat(0.42708890021612717) * t101 * t104 * t179) * t184 - f64x8::splat(0.01197423401025461) * t186 * t189 + (-f64x8::splat(0.031835665774679375) * t116 * t159 * t156 + f64x8::splat(0.05332506774217938) * t196 * t179) * t200 + f64x8::splat(0.020267214298646783) * t117 * t203 * t179 * t205;
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t20 * t23 * t73;
            let t214 = t213 * t213;
            let t215 = t214 * t214;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t209 * t216;
            let t218 = t34 * t34;
            let t219 = ((t33).select(t218, f64x8::splat(1.0)));
            let t220 = t219 * t219;
            let t221 = t220 * t219;
            let t222 = param_gamma * t221;
            let t223 = -t32 + t57 - t217;
            let t224 = f64x8::splat(1.0) / t179;
            let t226 = (simd::pow(t223 * t224, param_a_c));
            let t227 = param_beta * t226;
            let t228 = t227 * v_sigma;
            let t230 = f64x8::splat(1.0) / t7 / t195;
            let t231 = t230 * t39;
            let t232 = f64x8::splat(1.0) / t220;
            let t233 = t231 * t232;
            let t234 = t228 * t233;
            let t235 = f64x8::splat(1.0) / t3;
            let t236 = t18 * t235;
            let t237 = t236 * t5;
            let t238 = f64x8::splat(1.0) / param_gamma;
            let t242 = (simd::exp(-t223 / t221 * t238));
            let t243 = t242 - f64x8::splat(1.0);
            let t244 = f64x8::splat(1.0) / t243;
            let t245 = t238 * t244;
            let t247 = t227 * t245 * v_sigma;
            let t250 = t247 * t233 * t237 / f64x8::splat(96.0);
            let t251 = f64x8::splat(1.0) + t250;
            let t252 = t238 * t251;
            let t253 = param_beta * param_beta;
            let t254 = t226 * t226;
            let t255 = t253 * t254;
            let t256 = param_gamma * param_gamma;
            let t257 = f64x8::splat(1.0) / t256;
            let t258 = t243 * t243;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t257 * t259;
            let t261 = v_sigma * v_sigma;
            let t263 = t255 * t260 * t261;
            let t264 = t195 * t195;
            let t266 = f64x8::splat(1.0) / t21 / t264;
            let t267 = t266 * t129;
            let t268 = t220 * t220;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t267 * t269;
            let t271 = t1 * t167;
            let t272 = t271 * t6;
            let t273 = t270 * t272;
            let t276 = f64x8::splat(1.0) + t250 + t263 * t273 / f64x8::splat(3072.0);
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t252 * t277;
            let t279 = t237 * t278;
            let t282 = f64x8::splat(1.0) + t234 * t279 / f64x8::splat(96.0);
            let t283 = (simd::ln(t282));
            let t284 = t222 * t283;
            let tzk0 = -t32 + t57 - t217 + t284;
            acc_zk = tzk0;
            let t286 = t4 * t104 * t30;
            let t287 = f64x8::splat(0.0011073577833333333) * t286;
            let t288 = t26 * t26;
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t12 * t289;
            let t291 = f64x8::splat(1.0) / t13;
            let t292 = t291 * t1;
            let t293 = t76 * t103;
            let t294 = t292 * t293;
            let t296 = t4 * t104;
            let t298 = ((t10).sqrt());
            let t299 = t298 * t1;
            let t300 = t299 * t293;
            let t302 = t20 * t159;
            let t304 = -f64x8::splat(0.632975) * t294 - f64x8::splat(0.29896666666666666) * t296 - f64x8::splat(0.1023875) * t300 - f64x8::splat(0.08215666666666667) * t302;
            let t305 = f64x8::splat(1.0) / t29;
            let t306 = t304 * t305;
            let t307 = t290 * t306;
            let t308 = f64x8::splat(1.0) * t307;
            let t309 = t43 * t1;
            let t312 = t309 * t76 * t103 * t54;
            let t313 = f64x8::splat(0.0001831155503675316) * t312;
            let t314 = t43 * t45;
            let t315 = t50 * t50;
            let t316 = f64x8::splat(1.0) / t315;
            let t321 = -f64x8::splat(0.8630833333333333) * t294 - f64x8::splat(0.301925) * t296 - f64x8::splat(0.05501625) * t300 - f64x8::splat(0.082785) * t302;
            let t323 = f64x8::splat(1.0) / t53;
            let t324 = t316 * t321 * t323;
            let t325 = t314 * t324;
            let t326 = f64x8::splat(0.5848223397455204) * t325;
            let t328 = param_hyb_omega_0 * t291 * t1;
            let t330 = f64x8::splat(0.48717083333333333) * t328 * t293;
            let t334 = t80 * t13 * t1;
            let t337 = -t330 - t75 * t293 / f64x8::splat(12.0) - f64x8::splat(0.24484) * t334 * t293;
            let t339 = t88 * t88;
            let t340 = f64x8::splat(1.0) / t339;
            let t341 = t84 * t340;
            let t344 = -t330 - f64x8::splat(0.28737583333333333) * t85 * t293;
            let t347 = t60 * (t337 * t89 - t341 * t344);
            let t348 = f64x8::splat(1.0) / t84;
            let t350 = t348 * t88 * t94;
            let t358 = t6 * t230;
            let t361 = -f64x8::splat(0.001725) * t296 - f64x8::splat(0.013655) * t302 + f64x8::splat(0.0030486129349252553) * t196 - f64x8::splat(0.00046475) * t101 * t358;
            let t367 = t76 * t112;
            let t374 = t18 * t6;
            let t375 = t374 * t22;
            let t376 = t39 * t65;
            let t377 = t376 * t134;
            let t379 = f64x8::splat(0.0003956661414271145) * t375 * t377;
            let t380 = t1 * t8;
            let t383 = f64x8::splat(2.0) / f64x8::splat(45.0) * t126 * t380 * t134;
            let t384 = t133 * t133;
            let t385 = f64x8::splat(1.0) / t384;
            let t392 = -f64x8::splat(0.035991666666666665) * t4 * t104 * t39 - f64x8::splat(0.006666666666666667) * t20 * t159 * t129;
            let t393 = t385 * t392;
            let t396 = t126 * t127 * t393 / f64x8::splat(15.0);
            let t397 = t4 * t6;
            let t398 = t8 * t139;
            let t404 = f64x8::splat(0.4125) * t296 - t302 / f64x8::splat(6.0);
            let t405 = t139 * t404;
            let t410 = t379 + t383 - t396 + f64x8::splat(0.10821041362364843) * t397 * t398 * t142 + f64x8::splat(4.0) / f64x8::splat(3.0) * t405 * t144 + f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * f64x8::splat(M_PI);
            let t419 = t361 * t110;
            let t422 = t107 * t1 * t3;
            let t423 = t104 * t110;
            let t426 = t419 / f64x8::splat(2.0) + f64x8::splat(0.03135) * t422 * t423;
            let t429 = t5 * t203;
            let t433 = t22 * t161;
            let t439 = f64x8::splat(0.03233333333333333) * t296 - f64x8::splat(0.11266666666666666) * t302;
            let t441 = t161 * t439 * t1;
            let t444 = t168 * t8;
            let t449 = t379 + t383 - t396 + f64x8::splat(0.06077777777777778) * t237 * t433 * t164 + t441 * t169 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t166 * t444 - f64x8::splat(2.0) / f64x8::splat(45.0) * t172 * t380;
            let t456 = t287 + t308 - t313 - t326;
            let t462 = t116 * t429;
            let t465 = t419 * t188;
            let t468 = t195 * v_rho;
            let t469 = f64x8::splat(1.0) / t468;
            let t470 = t469 * t107;
            let t471 = t112 * t187;
            let t487 = f64x8::splat(1.0) / t21 / t468;
            let t496 = t347 * t350 + (f64x8::splat(0.031505407223141116) * t196 * t107 * t112 - f64x8::splat(0.031505407223141116) * t96 * t361 * t112 - f64x8::splat(0.001975389032890948) * t230 * t107 * t1 * t367 + f64x8::splat(0.008980675507690957) * t117 * t203 * t147 * t111 - f64x8::splat(0.005388405304614574) * t117 * t119 * t410 * t111) * t80 + (f64x8::splat(0.0837628205355044) * t196 * t156 - f64x8::splat(0.0837628205355044) * t96 * t426 + f64x8::splat(0.019897291109174608) * t116 * t429 * t175 - f64x8::splat(0.011938374665504766) * t116 * t159 * t449 - f64x8::splat(0.5694518669548363) * t101 * t358 * t179 + f64x8::splat(0.42708890021612717) * t101 * t104 * t456) * t184 + f64x8::splat(0.019957056683757683) * t462 * t189 - f64x8::splat(0.01197423401025461) * t186 * t465 - f64x8::splat(0.0002905674151788692) * t470 * t471 + (f64x8::splat(0.053059442957798957) * t116 * t429 * t156 - f64x8::splat(0.031835665774679375) * t116 * t159 * t426 - f64x8::splat(0.10665013548435875) * t469 * t179 + f64x8::splat(0.05332506774217938) * t196 * t456) * t200 - f64x8::splat(0.054045904796391424) * t117 * t487 * t179 * t205 + f64x8::splat(0.020267214298646783) * t117 * t203 * t456 * t205;
            let t497 = t496 * t216;
            let t499 = f64x8::splat(1.0) / t215 / t213;
            let t501 = t209 * t499 * t18;
            let t502 = t19 * t5;
            let t504 = t502 * t119 * t73;
            let t505 = t501 * t504;
            let t506 = f64x8::splat(0.41076328840066667) * t505;
            let t507 = t287 + t308 - t313 - t326 - t497 - t506;
            let t509 = t179 * t179;
            let t510 = f64x8::splat(1.0) / t509;
            let t511 = t223 * t510;
            let t513 = t507 * t224 - t511 * t456;
            let t514 = param_a_c * t513;
            let t515 = t227 * t514;
            let t516 = f64x8::splat(1.0) / t223;
            let t517 = t516 * t179;
            let t518 = v_sigma * t230;
            let t519 = t517 * t518;
            let t520 = t515 * t519;
            let t521 = t39 * t232;
            let t522 = t521 * t236;
            let t523 = t5 * t238;
            let t524 = t251 * t277;
            let t526 = t522 * t523 * t524;
            let t530 = f64x8::splat(1.0) / t7 / t468;
            let t531 = t530 * t39;
            let t532 = t531 * t232;
            let t533 = t228 * t532;
            let t536 = t227 * param_a_c;
            let t537 = t513 * t516;
            let t538 = t179 * t238;
            let t540 = t536 * t537 * t538;
            let t541 = t244 * v_sigma;
            let t542 = t541 * t231;
            let t543 = t232 * t18;
            let t544 = t235 * t5;
            let t545 = t543 * t544;
            let t546 = t542 * t545;
            let t548 = t540 * t546 / f64x8::splat(96.0);
            let t549 = t227 * t257;
            let t550 = t259 * v_sigma;
            let t551 = t550 * t230;
            let t552 = t549 * t551;
            let t553 = t268 * t219;
            let t554 = f64x8::splat(1.0) / t553;
            let t555 = t39 * t554;
            let t556 = t555 * t18;
            let t557 = t507 * t242;
            let t558 = t544 * t557;
            let t559 = t556 * t558;
            let t561 = t552 * t559 / f64x8::splat(96.0);
            let t564 = f64x8::splat(7.0) / f64x8::splat(288.0) * t247 * t532 * t237;
            let t565 = t548 + t561 - t564;
            let t566 = t238 * t565;
            let t568 = t237 * t566 * t277;
            let t571 = t276 * t276;
            let t572 = f64x8::splat(1.0) / t571;
            let t573 = t255 * t257;
            let t574 = t259 * t261;
            let t576 = t573 * t574 * t267;
            let t577 = t269 * t1;
            let t578 = t577 * t168;
            let t579 = t514 * t517;
            let t580 = t578 * t579;
            let t584 = f64x8::splat(1.0) / t256 / param_gamma;
            let t585 = t255 * t584;
            let t587 = f64x8::splat(1.0) / t258 / t243;
            let t588 = t587 * t261;
            let t589 = t588 * t266;
            let t590 = t585 * t589;
            let t592 = f64x8::splat(1.0) / t268 / t221;
            let t593 = t129 * t592;
            let t594 = t593 * t1;
            let t596 = t594 * t168 * t557;
            let t599 = t264 * v_rho;
            let t601 = f64x8::splat(1.0) / t21 / t599;
            let t602 = t601 * t129;
            let t603 = t602 * t269;
            let t604 = t603 * t272;
            let t607 = t548 + t561 - t564 + t576 * t580 / f64x8::splat(1536.0) + t590 * t596 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t263 * t604;
            let t608 = t572 * t607;
            let t609 = t252 * t608;
            let t610 = t237 * t609;
            let t613 = t520 * t526 / f64x8::splat(96.0) - f64x8::splat(7.0) / f64x8::splat(288.0) * t533 * t279 + t234 * t568 / f64x8::splat(96.0) - t234 * t610 / f64x8::splat(96.0);
            let t614 = f64x8::splat(1.0) / t282;
            let t616 = t222 * t613 * t614;
            let tvrho0 = -t32 + t57 - t217 + t284 + v_rho * (t287 + t308 - t313 - t326 - t497 - t506 + t616);
            acc_vrho = tvrho0;
            let t619 = v_rho * param_gamma;
            let t620 = t227 * t233;
            let t623 = t255 * v_sigma;
            let t624 = t623 * t270;
            let t625 = t257 * t244;
            let t627 = t272 * t625 * t277;
            let t632 = t521 * t237;
            let t635 = t260 * v_sigma;
            let t636 = t255 * t635;
            let t639 = t227 * t245 * t230 * t632 / f64x8::splat(96.0) + t636 * t273 / f64x8::splat(1536.0);
            let t640 = t572 * t639;
            let t641 = t252 * t640;
            let t642 = t237 * t641;
            let t645 = t620 * t279 / f64x8::splat(96.0) + t624 * t627 / f64x8::splat(3072.0) - t234 * t642 / f64x8::splat(96.0);
            let tvsigma0 = t619 * t221 * t645 * t614;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
