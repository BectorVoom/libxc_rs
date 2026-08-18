//! MGGA_X_MGGAC vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mggac.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mggac_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(32.0);
        let t22 = t21 * t21;
        let t23 = t5 * t5;
        let t24 = t22 * t23;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t19 * t19;
        let t30 = 1.0 / t28 / rho[ip];
        let t31 = t27 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = 1.0 / rho[ip];
        let t39 = sigma[ip] * t38;
        let t40 = 1.0 / tau[ip];
        let t42 = t39 * t40 / 8.0;
        let t44 = 0.0 < 0.9999999999 - t42;
        let t46 = piecewise3(t44, 1.0 - t42, 1e-10);
        let t47 = t37 * t46;
        let t48 = t31 * t47;
        let t50 = tau[ip] * tau[ip];
        let t51 = t50 * t25;
        let t52 = rho[ip] * rho[ip];
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t19 / t53;
        let t56 = t51 * t55;
        let t57 = t32 * t32;
        let t59 = 1.0 / t34 / t33;
        let t60 = t57 * t59;
        let t61 = t46 * t46;
        let t62 = t60 * t61;
        let t63 = t56 * t62;
        let t65 = 1.0 + 0.05555555555555555 * t48 - 6.972166666666666 * t63;
        let t68 = 3.712 + 1.1111111111111112 * t48 + 2.3240555555555558 * t63;
        let t69 = 1.0 / t68;
        let t73 = xc_mgga_x_mbrxc_get_x(t24 * t65 * t69 / 6.0);
        let t74 = pow_1_4(f64::EPSILON);
        let t75 = t73 < t74;
        let t76 = t21 * t5;
        let t77 = t4 * t4;
        let t78 = 1.0 / M_PI;
        let t79 = pow_1_3(t78);
        let t80 = 1.0 / t79;
        let t81 = t77 * t80;
        let t82 = M_CBRT4;
        let t84 = t76 * t81 * t82;
        let t86 = t76 * t77;
        let t87 = t80 * t82;
        let t88 = t73 * t73;
        let t89 = t87 * t88;
        let t92 = t88 * t73;
        let t93 = t87 * t92;
        let t96 = t88 * t88;
        let t97 = t87 * t96;
        let t100 = t96 * t73;
        let t101 = t87 * t100;
        let t104 = t96 * t88;
        let t105 = t87 * t104;
        let t113 = t76 * t81;
        let t114 = t74 < t73;
        let t115 = piecewise3(t114, t73, t74);
        let t117 = f64::exp(t115 / 3.0);
        let t118 = t82 * t117;
        let t119 = f64::exp(-t115);
        let t120 = t115 * t115;
        let t122 = t120 + 5.0 * t115 + 8.0;
        let t123 = t119 * t122;
        let t124 = 8.0 - t123;
        let t125 = 1.0 / t115;
        let t126 = t124 * t125;
        let t127 = 1.0 + t115;
        let t128 = pow_1_3(t127);
        let t129 = 1.0 / t128;
        let t130 = t126 * t129;
        let t134 = piecewise3(t75, -t84 / 12.0 - t86 * t89 / 108.0 + t86 * t93 / 108.0 - 13.0 / 1620.0 * t86 * t97 + 67.0 / 9720.0 * t86 * t101 - 52.0 / 8505.0 * t86 * t105 + 1811.0 / 326592.0 * t86 * t87 * t96 * t92, -t113 * t118 * t130 / 36.0);
        let t138 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t134);
        let tzk0 = 2.0 * t138;
        zk[ip] += tzk0;
        let t140 = t18 / t28;
        let t144 = t22 * t6;
        let t145 = t144 * t77;
        let t147 = t145 * t87 * t73;
        let t149 = 1.0 / t28 / t52;
        let t150 = t27 * t149;
        let t151 = t150 * t47;
        let t153 = 1.0 / t52;
        let t154 = sigma[ip] * t153;
        let t157 = piecewise3(t44, t154 * t40 / 8.0, 0.0);
        let t158 = t37 * t157;
        let t159 = t31 * t158;
        let t161 = t52 * t52;
        let t163 = 1.0 / t19 / t161;
        let t164 = t51 * t163;
        let t165 = t164 * t62;
        let t168 = t60 * t46 * t157;
        let t169 = t56 * t168;
        let t171 = -0.09259259259259259 * t151 + 0.05555555555555555 * t159 + 23.240555555555556 * t165 - 13.944333333333333 * t169;
        let t174 = t68 * t68;
        let t175 = 1.0 / t174;
        let t176 = t65 * t175;
        let t181 = -1.8518518518518519 * t151 + 1.1111111111111112 * t159 - 7.746851851851852 * t165 + 4.6481111111111115 * t169;
        let t185 = t24 * t171 * t69 / 6.0 - t24 * t176 * t181 / 6.0;
        let t186 = t65 * t65;
        let t187 = 1.0 / t186;
        let t188 = t185 * t187;
        let t189 = t188 * t174;
        let t190 = 1.0 + t73;
        let t191 = pow_1_3(t190);
        let t192 = t191 * t191;
        let t193 = 1.0 / t192;
        let t195 = f64::exp(-2.0 / 3.0 * t73);
        let t196 = 1.0 / t195;
        let t197 = t193 * t196;
        let t199 = t88 - 3.0 * t73 + 6.0;
        let t200 = 1.0 / t199;
        let t201 = t73 - 3.0;
        let t202 = t201 * t201;
        let t203 = t200 * t202;
        let t204 = t197 * t203;
        let t205 = t189 * t204;
        let t208 = t145 * t89;
        let t211 = t145 * t93;
        let t214 = t145 * t97;
        let t217 = t145 * t101;
        let t220 = t145 * t105;
        let t224 = 1.0 / t23;
        let t225 = t21 * t224;
        let t226 = t225 * t188;
        let t227 = t174 * t193;
        let t228 = t196 * t200;
        let t229 = t228 * t202;
        let t230 = t227 * t229;
        let t231 = t226 * t230;
        let t233 = piecewise3(t114, 9.0 / 32.0 * t231, 0.0);
        let t234 = t233 * t117;
        let t238 = t233 * t119;
        let t239 = t238 * t122;
        let t243 = 2.0 * t115 * t233 + 5.0 * t233;
        let t244 = t119 * t243;
        let t245 = t239 - t244;
        let t246 = t245 * t125;
        let t247 = t246 * t129;
        let t251 = t117 * t124;
        let t252 = 1.0 / t120;
        let t253 = t252 * t129;
        let t254 = t253 * t233;
        let t259 = 1.0 / t128 / t127;
        let t260 = t125 * t259;
        let t261 = t260 * t233;
        let t266 = piecewise3(t75, -t147 * t205 / 192.0 + t208 * t205 / 128.0 - 13.0 / 1440.0 * t211 * t205 + 67.0 / 6912.0 * t214 * t205 - 13.0 / 1260.0 * t217 * t205 + 1811.0 / 165888.0 * t220 * t205, -t84 * t234 * t130 / 108.0 - t113 * t118 * t247 / 36.0 + t84 * t251 * t254 / 36.0 + t84 * t251 * t261 / 108.0);
        let t271 = piecewise3(t3, 0.0, t7 * t140 * t134 / 16.0 + 3.0 / 16.0 * t7 * t20 * t266);
        let tvrho0 = 2.0 * rho[ip] * t271 + 2.0 * t138;
        vrho[ip] += tvrho0;
        let t276 = piecewise3(t44, -t38 * t40 / 8.0, 0.0);
        let t277 = t37 * t276;
        let t278 = t31 * t277;
        let t281 = t60 * t46 * t276;
        let t282 = t56 * t281;
        let t284 = 0.05555555555555555 * t278 - 13.944333333333333 * t282;
        let t289 = 1.1111111111111112 * t278 + 4.6481111111111115 * t282;
        let t293 = -t24 * t176 * t289 / 6.0 + t24 * t284 * t69 / 6.0;
        let t294 = t293 * t187;
        let t295 = t294 * t174;
        let t296 = t295 * t204;
        let t310 = t225 * t294;
        let t311 = t310 * t230;
        let t313 = piecewise3(t114, 9.0 / 32.0 * t311, 0.0);
        let t314 = t313 * t117;
        let t318 = t313 * t119;
        let t319 = t318 * t122;
        let t323 = 2.0 * t115 * t313 + 5.0 * t313;
        let t324 = t119 * t323;
        let t325 = t319 - t324;
        let t326 = t325 * t125;
        let t327 = t326 * t129;
        let t331 = t253 * t313;
        let t335 = t260 * t313;
        let t340 = piecewise3(t75, -t147 * t296 / 192.0 + t208 * t296 / 128.0 - 13.0 / 1440.0 * t211 * t296 + 67.0 / 6912.0 * t214 * t296 - 13.0 / 1260.0 * t217 * t296 + 1811.0 / 165888.0 * t220 * t296, -t84 * t314 * t130 / 108.0 - t113 * t118 * t327 / 36.0 + t84 * t251 * t331 / 36.0 + t84 * t251 * t335 / 108.0);
        let t344 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t340);
        let tvsigma0 = 2.0 * rho[ip] * t344;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t346 = t26 * t30;
        let t347 = t346 * t47;
        let t349 = 1.0 / t50;
        let t352 = piecewise3(t44, t39 * t349 / 8.0, 0.0);
        let t353 = t37 * t352;
        let t354 = t31 * t353;
        let t356 = tau[ip] * t25;
        let t357 = t356 * t55;
        let t358 = t357 * t62;
        let t361 = t60 * t46 * t352;
        let t362 = t56 * t361;
        let t364 = 0.05555555555555555 * t347 + 0.05555555555555555 * t354 - 13.944333333333333 * t358 - 13.944333333333333 * t362;
        let t371 = 1.1111111111111112 * t347 + 1.1111111111111112 * t354 + 4.6481111111111115 * t358 + 4.6481111111111115 * t362;
        let t375 = -t24 * t176 * t371 / 6.0 + t24 * t364 * t69 / 6.0;
        let t376 = t375 * t187;
        let t377 = t376 * t174;
        let t378 = t377 * t204;
        let t392 = t225 * t376;
        let t393 = t392 * t230;
        let t395 = piecewise3(t114, 9.0 / 32.0 * t393, 0.0);
        let t396 = t395 * t117;
        let t400 = t395 * t119;
        let t405 = 2.0 * t115 * t395 + 5.0 * t395;
        let t406 = t119 * t405;
        let t407 = t400 * t122 - t406;
        let t408 = t407 * t125;
        let t409 = t408 * t129;
        let t413 = t253 * t395;
        let t417 = t260 * t395;
        let t422 = piecewise3(t75, -t147 * t378 / 192.0 + t208 * t378 / 128.0 - 13.0 / 1440.0 * t211 * t378 + 67.0 / 6912.0 * t214 * t378 - 13.0 / 1260.0 * t217 * t378 + 1811.0 / 165888.0 * t220 * t378, -t84 * t396 * t130 / 108.0 - t113 * t118 * t409 / 36.0 + t84 * t251 * t413 / 36.0 + t84 * t251 * t417 / 108.0);
        let t426 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t422);
        let tvtau0 = 2.0 * rho[ip] * t426;
        vtau[ip] += tvtau0;
    }
}
