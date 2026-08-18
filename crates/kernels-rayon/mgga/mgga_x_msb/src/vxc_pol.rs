//! MGGA_X_MSB vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_msb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_msb_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_kappa: f64,
    param_b: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t42 = 5.0 / 972.0 * t34 * t40;
        let t43 = param_kappa + t42;
        let t47 = param_kappa * (1.0 - param_kappa / t43);
        let t49 = 1.0 / t37 / rho0;
        let t50 = tau0 * t49;
        let t52 = t50 - t40 / 8.0;
        let t53 = t52 * t52;
        let t54 = t29 * t29;
        let t56 = 3.0 / 10.0 * t54 * t32;
        let t57 = t50 + t56;
        let t58 = t57 * t57;
        let t59 = 1.0 / t58;
        let t62 = -4.0 * t53 * t59 + 1.0;
        let t63 = t62 * t62;
        let t64 = t63 * t62;
        let t65 = t53 * t52;
        let t66 = t58 * t57;
        let t67 = 1.0 / t66;
        let t70 = t53 * t53;
        let t72 = param_b * t70 * t53;
        let t73 = t58 * t58;
        let t75 = 1.0 / t73 / t58;
        let t78 = 8.0 * t65 * t67 + 64.0 * t72 * t75 + 1.0;
        let t79 = 1.0 / t78;
        let t80 = t64 * t79;
        let t81 = param_kappa + t42 + param_c;
        let t86 = param_kappa * (1.0 - param_kappa / t81) - t47;
        let t88 = t80 * t86 + t47 + 1.0;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t88);
        let t93 = rho1 <= dens_threshold;
        let t94 = -t17;
        let t96 = piecewise5(t15, t12, t11, t16, t94 * t8);
        let t97 = 1.0 + t96;
        let t98 = t97 <= zeta_threshold;
        let t99 = pow_1_3(t97);
        let t101 = piecewise3(t98, t23, t99 * t97);
        let t102 = t101 * t27;
        let t103 = rho1 * rho1;
        let t104 = pow_1_3(rho1);
        let t105 = t104 * t104;
        let t107 = 1.0 / t105 / t103;
        let t108 = sigma2 * t107;
        let t110 = 5.0 / 972.0 * t34 * t108;
        let t111 = param_kappa + t110;
        let t115 = param_kappa * (1.0 - param_kappa / t111);
        let t117 = 1.0 / t105 / rho1;
        let t118 = tau1 * t117;
        let t120 = t118 - t108 / 8.0;
        let t121 = t120 * t120;
        let t122 = t118 + t56;
        let t123 = t122 * t122;
        let t124 = 1.0 / t123;
        let t127 = -4.0 * t121 * t124 + 1.0;
        let t128 = t127 * t127;
        let t129 = t128 * t127;
        let t130 = t121 * t120;
        let t131 = t123 * t122;
        let t132 = 1.0 / t131;
        let t135 = t121 * t121;
        let t137 = param_b * t135 * t121;
        let t138 = t123 * t123;
        let t140 = 1.0 / t138 / t123;
        let t143 = 8.0 * t130 * t132 + 64.0 * t137 * t140 + 1.0;
        let t144 = 1.0 / t143;
        let t145 = t129 * t144;
        let t146 = param_kappa + t110 + param_c;
        let t151 = param_kappa * (1.0 - param_kappa / t146) - t115;
        let t153 = t145 * t151 + t115 + 1.0;
        let t157 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t102 * t153);
        let tzk0 = t92 + t157;
        zk[ip] += tzk0;
        let t158 = t7 * t7;
        let t159 = 1.0 / t158;
        let t160 = t17 * t159;
        let t162 = piecewise5(t11, 0.0, t15, 0.0, t8 - t160);
        let t165 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t162);
        let t166 = t165 * t27;
        let t170 = t27 * t27;
        let t171 = 1.0 / t170;
        let t172 = t26 * t171;
        let t175 = t6 * t172 * t88 / 8.0;
        let t176 = param_kappa * param_kappa;
        let t177 = t43 * t43;
        let t179 = t176 / t177;
        let t180 = t179 * t29;
        let t181 = t33 * sigma0;
        let t182 = t35 * rho0;
        let t184 = 1.0 / t37 / t182;
        let t185 = t181 * t184;
        let t186 = t180 * t185;
        let t188 = t63 * t79;
        let t189 = t52 * t59;
        let t190 = tau0 * t39;
        let t194 = -5.0 / 3.0 * t190 + sigma0 * t184 / 3.0;
        let t197 = t53 * t67;
        let t200 = -8.0 * t189 * t194 - 40.0 / 3.0 * t197 * t190;
        let t201 = t86 * t200;
        let t204 = t78 * t78;
        let t205 = 1.0 / t204;
        let t206 = t64 * t205;
        let t209 = 1.0 / t73;
        let t210 = t65 * t209;
        let t214 = param_b * t70 * t52;
        let t215 = t75 * t194;
        let t219 = 1.0 / t73 / t66;
        let t220 = t219 * tau0;
        let t224 = 640.0 * t72 * t220 * t39 + 40.0 * t210 * t190 + 24.0 * t197 * t194 + 384.0 * t214 * t215;
        let t225 = t86 * t224;
        let t227 = t81 * t81;
        let t229 = t176 / t227;
        let t230 = t229 * t29;
        let t233 = -10.0 / 729.0 * t230 * t185 + 10.0 / 729.0 * t186;
        let t235 = -10.0 / 729.0 * t186 + 3.0 * t188 * t201 - t206 * t225 + t80 * t233;
        let t240 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t166 * t88 - t175 - 3.0 / 8.0 * t6 * t28 * t235);
        let t241 = t94 * t159;
        let t243 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t241);
        let t246 = piecewise3(t98, 0.0, 4.0 / 3.0 * t99 * t243);
        let t247 = t246 * t27;
        let t251 = t101 * t171;
        let t254 = t6 * t251 * t153 / 8.0;
        let t256 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t247 * t153 - t254);
        let tvrho0 = t92 + t157 + t7 * (t240 + t256);
        vrho[ip * 2] += tvrho0;
        let t260 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t160);
        let t263 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t260);
        let t264 = t263 * t27;
        let t269 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t264 * t88 - t175);
        let t271 = piecewise5(t15, 0.0, t11, 0.0, t8 - t241);
        let t274 = piecewise3(t98, 0.0, 4.0 / 3.0 * t99 * t271);
        let t275 = t274 * t27;
        let t279 = t111 * t111;
        let t281 = t176 / t279;
        let t282 = t281 * t29;
        let t283 = t33 * sigma2;
        let t284 = t103 * rho1;
        let t286 = 1.0 / t105 / t284;
        let t287 = t283 * t286;
        let t288 = t282 * t287;
        let t290 = t128 * t144;
        let t291 = t120 * t124;
        let t292 = tau1 * t107;
        let t296 = -5.0 / 3.0 * t292 + sigma2 * t286 / 3.0;
        let t299 = t121 * t132;
        let t302 = -8.0 * t291 * t296 - 40.0 / 3.0 * t299 * t292;
        let t303 = t151 * t302;
        let t306 = t143 * t143;
        let t307 = 1.0 / t306;
        let t308 = t129 * t307;
        let t311 = 1.0 / t138;
        let t312 = t130 * t311;
        let t316 = param_b * t135 * t120;
        let t317 = t140 * t296;
        let t321 = 1.0 / t138 / t131;
        let t322 = t321 * tau1;
        let t326 = 640.0 * t137 * t322 * t107 + 40.0 * t312 * t292 + 24.0 * t299 * t296 + 384.0 * t316 * t317;
        let t327 = t151 * t326;
        let t329 = t146 * t146;
        let t331 = t176 / t329;
        let t332 = t331 * t29;
        let t335 = -10.0 / 729.0 * t332 * t287 + 10.0 / 729.0 * t288;
        let t337 = -10.0 / 729.0 * t288 + 3.0 * t290 * t303 - t308 * t327 + t145 * t335;
        let t342 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t275 * t153 - t254 - 3.0 / 8.0 * t6 * t102 * t337);
        let tvrho1 = t92 + t157 + t7 * (t269 + t342);
        vrho[ip * 2 + 1] += tvrho1;
        let t345 = t34 * t39;
        let t346 = t179 * t345;
        let t348 = t188 * t86;
        let t349 = t189 * t39;
        let t352 = t197 * t39;
        let t354 = t75 * t39;
        let t355 = t214 * t354;
        let t357 = -3.0 * t352 - 48.0 * t355;
        let t358 = t86 * t357;
        let t362 = 5.0 / 972.0 * t229 * t345 - 5.0 / 972.0 * t346;
        let t364 = 5.0 / 972.0 * t346 + 3.0 * t348 * t349 - t206 * t358 + t80 * t362;
        let t368 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t364);
        let tvsigma0 = t7 * t368;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t369 = t34 * t107;
        let t370 = t281 * t369;
        let t372 = t290 * t151;
        let t373 = t291 * t107;
        let t376 = t299 * t107;
        let t378 = t140 * t107;
        let t379 = t316 * t378;
        let t381 = -3.0 * t376 - 48.0 * t379;
        let t382 = t151 * t381;
        let t386 = 5.0 / 972.0 * t331 * t369 - 5.0 / 972.0 * t370;
        let t388 = 5.0 / 972.0 * t370 + 3.0 * t372 * t373 - t308 * t382 + t145 * t386;
        let t392 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t102 * t388);
        let tvsigma2 = t7 * t392;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t394 = t197 * t49;
        let t396 = -8.0 * t189 * t49 + 8.0 * t394;
        let t397 = t86 * t396;
        let t403 = t75 * t49;
        let t406 = t219 * t49;
        let t409 = -24.0 * t210 * t49 + 384.0 * t214 * t403 - 384.0 * t72 * t406 + 24.0 * t394;
        let t410 = t86 * t409;
        let t412 = 3.0 * t188 * t397 - t206 * t410;
        let t416 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t412);
        let tvtau0 = t7 * t416;
        vtau[ip * 2] += tvtau0;
        let t418 = t299 * t117;
        let t420 = -8.0 * t291 * t117 + 8.0 * t418;
        let t421 = t151 * t420;
        let t427 = t140 * t117;
        let t430 = t321 * t117;
        let t433 = -24.0 * t312 * t117 - 384.0 * t137 * t430 + 384.0 * t316 * t427 + 24.0 * t418;
        let t434 = t151 * t433;
        let t436 = 3.0 * t290 * t421 - t308 * t434;
        let t440 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t102 * t436);
        let tvtau1 = t7 * t440;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
