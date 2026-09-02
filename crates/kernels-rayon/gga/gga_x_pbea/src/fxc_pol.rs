//! GGA_X_PBEA fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbea_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t28 = rho0 * rho0;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / t28;
        let t35 = 1.0 + 0.008639940809536326 * sigma0 * t32;
        let t36 = rmath::pow(t35, -0.52);
        let t38 = 1.804 - 0.804 * t36;
        let t42 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t38);
        let t43 = rho1 <= dens_threshold;
        let t44 = -t16;
        let t46 = piecewise5(t14, t11, t10, t15, t44 * t7);
        let t47 = 1.0 + t46;
        let t48 = t47 <= zeta_threshold;
        let t49 = pow_1_3(t47);
        let t51 = piecewise3(t48, t22, t49 * t47);
        let t53 = rho1 * rho1;
        let t54 = pow_1_3(rho1);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / t53;
        let t60 = 1.0 + 0.008639940809536326 * sigma2 * t57;
        let t61 = rmath::pow(t60, -0.52);
        let t63 = 1.804 - 0.804 * t61;
        let t67 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t51 * t26 * t63);
        let tzk0 = t42 + t67;
        zk[ip] += tzk0;
        let t68 = t6 * t6;
        let t69 = 1.0 / t68;
        let t70 = t16 * t69;
        let t72 = piecewise5(t10, 0.0, t14, 0.0, t7 - t70);
        let t75 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t72);
        let t80 = t26 * t26;
        let t81 = 1.0 / t80;
        let t85 = t5 * t25 * t81 * t38 / 8.0;
        let t86 = t2 * t25;
        let t87 = t86 * t26;
        let t88 = rmath::pow(t35, -1.52);
        let t89 = t88 * sigma0;
        let t90 = t28 * rho0;
        let t92 = 1.0 / t30 / t90;
        let t93 = t89 * t92;
        let t97 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t75 * t26 * t38 - t85 + 0.00246634334405953 * t87 * t93);
        let t98 = t44 * t69;
        let t100 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t98);
        let t103 = piecewise3(t48, 0.0, 4.0 / 3.0 * t49 * t100);
        let t111 = t5 * t51 * t81 * t63 / 8.0;
        let t113 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t103 * t26 * t63 - t111);
        let tvrho0 = t42 + t67 + t6 * (t97 + t113);
        vrho[ip * 2] += tvrho0;
        let t117 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t70);
        let t120 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t117);
        let t126 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t120 * t26 * t38 - t85);
        let t128 = piecewise5(t14, 0.0, t10, 0.0, t7 - t98);
        let t131 = piecewise3(t48, 0.0, 4.0 / 3.0 * t49 * t128);
        let t136 = t2 * t51;
        let t137 = t136 * t26;
        let t138 = rmath::pow(t60, -1.52);
        let t139 = t138 * sigma2;
        let t140 = t53 * rho1;
        let t142 = 1.0 / t55 / t140;
        let t143 = t139 * t142;
        let t147 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t131 * t26 * t63 - t111 + 0.00246634334405953 * t137 * t143);
        let tvrho1 = t42 + t67 + t6 * (t126 + t147);
        vrho[ip * 2 + 1] += tvrho1;
        let t150 = t26 * t88;
        let t151 = t150 * t32;
        let t154 = piecewise3(t1, 0.0, -0.0009248787540223239 * t86 * t151);
        let tvsigma0 = t6 * t154;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t155 = t26 * t138;
        let t156 = t155 * t57;
        let t159 = piecewise3(t43, 0.0, -0.0009248787540223239 * t136 * t156);
        let tvsigma2 = t6 * t159;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t162 = t23 * t23;
        let t163 = 1.0 / t162;
        let t164 = t72 * t72;
        let t167 = t68 * t6;
        let t168 = 1.0 / t167;
        let t169 = t16 * t168;
        let t172 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t69 + 2.0 * t169);
        let t176 = piecewise3(t20, 0.0, 4.0 / 9.0 * t163 * t164 + 4.0 / 3.0 * t23 * t172);
        let t183 = t5 * t75 * t81 * t38;
        let t185 = t2 * t75;
        let t186 = t185 * t26;
        let t190 = 1.0 / t80 / t6;
        let t194 = t5 * t25 * t190 * t38 / 12.0;
        let t195 = t86 * t81;
        let t196 = t195 * t93;
        let t198 = rmath::pow(t35, -2.52);
        let t199 = sigma0 * sigma0;
        let t200 = t198 * t199;
        let t201 = t28 * t28;
        let t204 = 1.0 / t29 / t201 / t90;
        let t205 = t200 * t204;
        let t209 = 1.0 / t30 / t201;
        let t210 = t89 * t209;
        let t214 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t176 * t26 * t38 - t183 / 4.0 + 0.00493268668811906 * t186 * t93 + t194 + 0.0016442288960396869 * t196 + 8.637272526180187e-05 * t87 * t205 - 0.009043258928218278 * t87 * t210);
        let t215 = t49 * t49;
        let t216 = 1.0 / t215;
        let t217 = t100 * t100;
        let t220 = t44 * t168;
        let t223 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t69 + 2.0 * t220);
        let t227 = piecewise3(t48, 0.0, 4.0 / 9.0 * t216 * t217 + 4.0 / 3.0 * t49 * t223);
        let t234 = t5 * t103 * t81 * t63;
        let t239 = t5 * t51 * t190 * t63 / 12.0;
        let t241 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t227 * t26 * t63 - t234 / 4.0 + t239);
        let tv2rho20 = 2.0 * t97 + 2.0 * t113 + t6 * (t214 + t241);
        v2rho2[ip * 3] += tv2rho20;
        let t244 = t163 * t117;
        let t248 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t169);
        let t252 = piecewise3(t20, 0.0, 4.0 / 9.0 * t244 * t72 + 4.0 / 3.0 * t23 * t248);
        let t259 = t5 * t120 * t81 * t38;
        let t261 = t2 * t120;
        let t262 = t261 * t26;
        let t268 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t252 * t26 * t38 - t259 / 8.0 + 0.00246634334405953 * t262 * t93 - t183 / 8.0 + t194 + 0.0008221144480198434 * t196);
        let t269 = t216 * t128;
        let t273 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t220);
        let t277 = piecewise3(t48, 0.0, 4.0 / 9.0 * t269 * t100 + 4.0 / 3.0 * t49 * t273);
        let t284 = t5 * t131 * t81 * t63;
        let t287 = t2 * t103;
        let t288 = t287 * t26;
        let t291 = t136 * t81;
        let t292 = t291 * t143;
        let t295 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t277 * t26 * t63 - t284 / 8.0 - t234 / 8.0 + t239 + 0.00246634334405953 * t288 * t143 + 0.0008221144480198434 * t292);
        let tv2rho21 = t97 + t113 + t126 + t147 + t6 * (t268 + t295);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t300 = t117 * t117;
        let t305 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t69 + 2.0 * t169);
        let t309 = piecewise3(t20, 0.0, 4.0 / 9.0 * t163 * t300 + 4.0 / 3.0 * t23 * t305);
        let t316 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t309 * t26 * t38 - t259 / 4.0 + t194);
        let t317 = t128 * t128;
        let t322 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t69 + 2.0 * t220);
        let t326 = piecewise3(t48, 0.0, 4.0 / 9.0 * t216 * t317 + 4.0 / 3.0 * t49 * t322);
        let t332 = t2 * t131;
        let t333 = t332 * t26;
        let t337 = rmath::pow(t60, -2.52);
        let t338 = sigma2 * sigma2;
        let t339 = t337 * t338;
        let t340 = t53 * t53;
        let t343 = 1.0 / t54 / t340 / t140;
        let t344 = t339 * t343;
        let t348 = 1.0 / t55 / t340;
        let t349 = t139 * t348;
        let t353 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t326 * t26 * t63 - t284 / 4.0 + 0.00493268668811906 * t333 * t143 + t239 + 0.0016442288960396869 * t292 + 8.637272526180187e-05 * t137 * t344 - 0.009043258928218278 * t137 * t349);
        let tv2rho22 = 2.0 * t126 + 2.0 * t147 + t6 * (t316 + t353);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t358 = t81 * t88;
        let t359 = t358 * t32;
        let t361 = 0.00030829291800744127 * t86 * t359;
        let t362 = t201 * t28;
        let t364 = 1.0 / t29 / t362;
        let t366 = t198 * t364 * sigma0;
        let t369 = t150 * t92;
        let t373 = piecewise3(t1, 0.0, -0.0009248787540223239 * t185 * t151 - t361 - 3.23897719731757e-05 * t87 * t366 + 0.00246634334405953 * t86 * t369);
        let tv2rhosigma0 = t6 * t373 + t154;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t377 = t81 * t138;
        let t378 = t377 * t57;
        let t380 = 0.00030829291800744127 * t136 * t378;
        let t382 = piecewise3(t43, 0.0, -0.0009248787540223239 * t287 * t156 - t380);
        let tv2rhosigma2 = t6 * t382 + t159;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t387 = piecewise3(t1, 0.0, -0.0009248787540223239 * t261 * t151 - t361);
        let tv2rhosigma3 = t6 * t387 + t154;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t391 = t340 * t53;
        let t393 = 1.0 / t54 / t391;
        let t395 = t337 * t393 * sigma2;
        let t398 = t155 * t142;
        let t402 = piecewise3(t43, 0.0, -0.0009248787540223239 * t332 * t156 - t380 - 3.23897719731757e-05 * t137 * t395 + 0.00246634334405953 * t136 * t398);
        let tv2rhosigma5 = t6 * t402 + t159;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t404 = t26 * t198;
        let t405 = t201 * rho0;
        let t407 = 1.0 / t29 / t405;
        let t408 = t404 * t407;
        let t411 = piecewise3(t1, 0.0, 1.214616448994089e-05 * t86 * t408);
        let tv2sigma20 = t6 * t411;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t412 = t26 * t337;
        let t413 = t340 * rho1;
        let t415 = 1.0 / t54 / t413;
        let t416 = t412 * t415;
        let t419 = piecewise3(t43, 0.0, 1.214616448994089e-05 * t136 * t416);
        let tv2sigma25 = t6 * t419;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
