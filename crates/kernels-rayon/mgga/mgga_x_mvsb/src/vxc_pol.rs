//! MGGA_X_MVSB vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvsb_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t33 = tau0 * t32;
        let t34 = rho0 * rho0;
        let t36 = 1.0 / t30 / t34;
        let t39 = t33 - sigma0 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t41 = t40 * t40;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t46 = 3.0 / 10.0 * t41 * t44;
        let t47 = t33 - t46;
        let t48 = 1.0 / t47;
        let t51 = param_k0 * (-t39 * t48 + 1.0);
        let t52 = t39 * t39;
        let t53 = param_e1 * t52;
        let t54 = t47 * t47;
        let t55 = 1.0 / t54;
        let t57 = t53 * t55 + 1.0;
        let t58 = t57 * t57;
        let t59 = t52 * t52;
        let t60 = param_c1 * t59;
        let t61 = t54 * t54;
        let t62 = 1.0 / t61;
        let t64 = t60 * t62 + t58;
        let t65 = pow_1_4(t64);
        let t66 = 1.0 / t65;
        let t68 = t51 * t66 + 1.0;
        let t70 = param_b * t41;
        let t72 = 1.0 / t43 / t42;
        let t73 = sigma0 * sigma0;
        let t74 = t72 * t73;
        let t75 = t34 * t34;
        let t76 = t75 * rho0;
        let t78 = 1.0 / t29 / t76;
        let t82 = 1.0 + t70 * t74 * t78 / 576.0;
        let t83 = rmath::pow(t82, 1.0 / 8.0);
        let t84 = 1.0 / t83;
        let t85 = t28 * t68 * t84;
        let t88 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t85);
        let t89 = rho1 <= dens_threshold;
        let t90 = -t17;
        let t92 = piecewise5(t15, t12, t11, t16, t90 * t8);
        let t93 = 1.0 + t92;
        let t94 = t93 <= zeta_threshold;
        let t95 = pow_1_3(t93);
        let t97 = piecewise3(t94, t23, t95 * t93);
        let t98 = t6 * t97;
        let t99 = pow_1_3(rho1);
        let t100 = t99 * t99;
        let t102 = 1.0 / t100 / rho1;
        let t103 = tau1 * t102;
        let t104 = rho1 * rho1;
        let t106 = 1.0 / t100 / t104;
        let t109 = t103 - sigma2 * t106 / 8.0;
        let t110 = t103 - t46;
        let t111 = 1.0 / t110;
        let t114 = param_k0 * (-t109 * t111 + 1.0);
        let t115 = t109 * t109;
        let t116 = param_e1 * t115;
        let t117 = t110 * t110;
        let t118 = 1.0 / t117;
        let t120 = t116 * t118 + 1.0;
        let t121 = t120 * t120;
        let t122 = t115 * t115;
        let t123 = param_c1 * t122;
        let t124 = t117 * t117;
        let t125 = 1.0 / t124;
        let t127 = t123 * t125 + t121;
        let t128 = pow_1_4(t127);
        let t129 = 1.0 / t128;
        let t131 = t114 * t129 + 1.0;
        let t133 = sigma2 * sigma2;
        let t134 = t72 * t133;
        let t135 = t104 * t104;
        let t136 = t135 * rho1;
        let t138 = 1.0 / t99 / t136;
        let t142 = 1.0 + t70 * t134 * t138 / 576.0;
        let t143 = rmath::pow(t142, 1.0 / 8.0);
        let t144 = 1.0 / t143;
        let t145 = t28 * t131 * t144;
        let t148 = piecewise3(t89, 0.0, -3.0 / 8.0 * t98 * t145);
        let tzk0 = t88 + t148;
        zk[ip] += tzk0;
        let t149 = t7 * t7;
        let t150 = 1.0 / t149;
        let t151 = t17 * t150;
        let t153 = piecewise5(t11, 0.0, t15, 0.0, t8 - t151);
        let t156 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t153);
        let t157 = t6 * t156;
        let t160 = t28 * t28;
        let t161 = 1.0 / t160;
        let t163 = t161 * t68 * t84;
        let t165 = t27 * t163 / 8.0;
        let t166 = tau0 * t36;
        let t168 = t34 * rho0;
        let t170 = 1.0 / t30 / t168;
        let t173 = -5.0 / 3.0 * t166 + sigma0 * t170 / 3.0;
        let t175 = t39 * t55;
        let t179 = param_k0 * (-t173 * t48 - 5.0 / 3.0 * t175 * t166);
        let t182 = 1.0 / t65 / t64;
        let t183 = param_e1 * t39;
        let t184 = t55 * t173;
        let t187 = t54 * t47;
        let t188 = 1.0 / t187;
        let t189 = t188 * tau0;
        let t190 = t189 * t36;
        let t193 = 2.0 * t183 * t184 + 10.0 / 3.0 * t53 * t190;
        let t197 = param_c1 * t52 * t39;
        let t198 = t62 * t173;
        let t202 = 1.0 / t61 / t47;
        let t203 = t202 * tau0;
        let t207 = 2.0 * t57 * t193 + 4.0 * t197 * t198 + 20.0 / 3.0 * t60 * t203 * t36;
        let t208 = t182 * t207;
        let t211 = t179 * t66 - t51 * t208 / 4.0;
        let t213 = t28 * t211 * t84;
        let t216 = t26 * t28;
        let t217 = t216 * t68;
        let t218 = t6 * t217;
        let t221 = 1.0 / t83 / t82 * param_b;
        let t222 = t221 * t41;
        let t223 = t75 * t34;
        let t225 = 1.0 / t29 / t223;
        let t227 = t222 * t74 * t225;
        let t231 = piecewise3(t2, 0.0, -3.0 / 8.0 * t157 * t85 - t165 - 3.0 / 8.0 * t27 * t213 - t218 * t227 / 2304.0);
        let t232 = t90 * t150;
        let t234 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t232);
        let t237 = piecewise3(t94, 0.0, 4.0 / 3.0 * t95 * t234);
        let t238 = t6 * t237;
        let t242 = t161 * t131 * t144;
        let t244 = t98 * t242 / 8.0;
        let t246 = piecewise3(t89, 0.0, -3.0 / 8.0 * t238 * t145 - t244);
        let tvrho0 = t88 + t148 + t7 * (t231 + t246);
        vrho[ip * 2] += tvrho0;
        let t250 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t151);
        let t253 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t250);
        let t254 = t6 * t253;
        let t258 = piecewise3(t2, 0.0, -3.0 / 8.0 * t254 * t85 - t165);
        let t260 = piecewise5(t15, 0.0, t11, 0.0, t8 - t232);
        let t263 = piecewise3(t94, 0.0, 4.0 / 3.0 * t95 * t260);
        let t264 = t6 * t263;
        let t267 = tau1 * t106;
        let t269 = t104 * rho1;
        let t271 = 1.0 / t100 / t269;
        let t274 = -5.0 / 3.0 * t267 + sigma2 * t271 / 3.0;
        let t276 = t109 * t118;
        let t280 = param_k0 * (-t274 * t111 - 5.0 / 3.0 * t276 * t267);
        let t283 = 1.0 / t128 / t127;
        let t284 = param_e1 * t109;
        let t285 = t118 * t274;
        let t288 = t117 * t110;
        let t289 = 1.0 / t288;
        let t290 = t289 * tau1;
        let t291 = t290 * t106;
        let t294 = 2.0 * t284 * t285 + 10.0 / 3.0 * t116 * t291;
        let t298 = param_c1 * t115 * t109;
        let t299 = t125 * t274;
        let t303 = 1.0 / t124 / t110;
        let t304 = t303 * tau1;
        let t308 = 2.0 * t120 * t294 + 4.0 * t298 * t299 + 20.0 / 3.0 * t123 * t304 * t106;
        let t309 = t283 * t308;
        let t312 = t280 * t129 - t114 * t309 / 4.0;
        let t314 = t28 * t312 * t144;
        let t317 = t97 * t28;
        let t318 = t317 * t131;
        let t319 = t6 * t318;
        let t322 = 1.0 / t143 / t142 * param_b;
        let t323 = t322 * t41;
        let t324 = t135 * t104;
        let t326 = 1.0 / t99 / t324;
        let t328 = t323 * t134 * t326;
        let t332 = piecewise3(t89, 0.0, -3.0 / 8.0 * t264 * t145 - t244 - 3.0 / 8.0 * t98 * t314 - t319 * t328 / 2304.0);
        let tvrho1 = t88 + t148 + t7 * (t258 + t332);
        vrho[ip * 2 + 1] += tvrho1;
        let t335 = param_k0 * t36;
        let t336 = t48 * t66;
        let t339 = t57 * param_e1;
        let t340 = t175 * t36;
        let t342 = t62 * t36;
        let t343 = t197 * t342;
        let t345 = -t339 * t340 / 2.0 - t343 / 2.0;
        let t346 = t182 * t345;
        let t349 = t335 * t336 / 8.0 - t51 * t346 / 4.0;
        let t351 = t28 * t349 * t84;
        let t354 = t72 * sigma0;
        let t356 = t222 * t354 * t78;
        let t360 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t351 + t218 * t356 / 6144.0);
        let tvsigma0 = t7 * t360;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t361 = param_k0 * t106;
        let t362 = t111 * t129;
        let t365 = t120 * param_e1;
        let t366 = t276 * t106;
        let t368 = t125 * t106;
        let t369 = t298 * t368;
        let t371 = -t365 * t366 / 2.0 - t369 / 2.0;
        let t372 = t283 * t371;
        let t375 = t361 * t362 / 8.0 - t114 * t372 / 4.0;
        let t377 = t28 * t375 * t144;
        let t380 = t72 * sigma2;
        let t382 = t323 * t380 * t138;
        let t386 = piecewise3(t89, 0.0, -3.0 / 8.0 * t98 * t377 + t319 * t382 / 6144.0);
        let tvsigma2 = t7 * t386;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t390 = param_k0 * (t175 * t32 - t32 * t48);
        let t392 = t55 * t32;
        let t394 = t188 * t32;
        let t397 = 2.0 * t183 * t392 - 2.0 * t53 * t394;
        let t400 = t62 * t32;
        let t403 = t202 * t32;
        let t406 = 4.0 * t197 * t400 + 2.0 * t57 * t397 - 4.0 * t60 * t403;
        let t407 = t182 * t406;
        let t410 = t390 * t66 - t51 * t407 / 4.0;
        let t412 = t28 * t410 * t84;
        let t415 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t412);
        let tvtau0 = t7 * t415;
        vtau[ip * 2] += tvtau0;
        let t419 = param_k0 * (-t102 * t111 + t276 * t102);
        let t421 = t118 * t102;
        let t423 = t289 * t102;
        let t426 = -2.0 * t116 * t423 + 2.0 * t284 * t421;
        let t429 = t125 * t102;
        let t432 = t303 * t102;
        let t435 = 2.0 * t120 * t426 - 4.0 * t123 * t432 + 4.0 * t298 * t429;
        let t436 = t283 * t435;
        let t439 = t419 * t129 - t114 * t436 / 4.0;
        let t441 = t28 * t439 * t144;
        let t444 = piecewise3(t89, 0.0, -3.0 / 8.0 * t98 * t441);
        let tvtau1 = t7 * t444;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
