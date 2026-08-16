//! MGGA_X_VT84 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_vt84_vxc_unpol(
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
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = sigma[ip] * sigma[ip];
        let t22 = t21 * sigma[ip];
        let t23 = rho[ip] * rho[ip];
        let t24 = t23 * rho[ip];
        let t25 = 1.0 / t24;
        let t26 = t22 * t25;
        let t27 = tau[ip] * tau[ip];
        let t28 = t27 * tau[ip];
        let t29 = 1.0 / t28;
        let t30 = 1.0 / t23;
        let t31 = t21 * t30;
        let t32 = 1.0 / t27;
        let t33 = t31 * t32;
        let t35 = 1.0 + t33 / 64.0;
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t29 * t37;
        let t42 = M_CBRT6;
        let t43 = (10.0 / 81.0 + 0.419826171875e-2 * t26 * t38) * t42;
        let t44 = M_PI * M_PI;
        let t45 = pow_1_3(t44);
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t48 = t43 * t47;
        let t49 = M_CBRT2;
        let t50 = t49 * t49;
        let t51 = sigma[ip] * t50;
        let t52 = t19 * t19;
        let t54 = 1.0 / t52 / t23;
        let t55 = t51 * t54;
        let t58 = tau[ip] * t50;
        let t60 = 1.0 / t52 / rho[ip];
        let t63 = t58 * t60 - t55 / 8.0;
        let t64 = t63 * t42;
        let t67 = 5.0 / 9.0 * t64 * t47 - 1.0;
        let t68 = t47 * t67;
        let t71 = 1.0 + 0.22222222222222222222e0 * t64 * t68;
        let t72 = f64::sqrt(t71);
        let t73 = 1.0 / t72;
        let t76 = t42 * t47;
        let t77 = t76 * t55;
        let t79 = 9.0 / 20.0 * t67 * t73 + t77 / 36.0;
        let t80 = t79 * t79;
        let t83 = t42 * t42;
        let t85 = 1.0 / t45 / t44;
        let t86 = t83 * t85;
        let t87 = t21 * t49;
        let t88 = t23 * t23;
        let t89 = t88 * rho[ip];
        let t91 = 1.0 / t19 / t89;
        let t93 = t86 * t87 * t91;
        let t95 = 162.0 * t33 + 100.0 * t93;
        let t96 = f64::sqrt(t95);
        let t101 = t88 * t88;
        let t102 = 1.0 / t101;
        let t105 = t48 * t55 / 24.0 + 146.0 / 2025.0 * t80 - 73.0 / 97200.0 * t79 * t96 + 0.5301186990888922759e-4 * t93 + 0.19577914932045745128e-2 * t33 + 0.43721079261097766676e-5 * t22 * t102;
        let t107 = 1.0 + 0.58733744796137235383e-1 * t77;
        let t108 = t107 * t107;
        let t109 = 1.0 / t108;
        let t110 = t105 * t109;
        let t112 = f64::exp(-0.1863e-3 * t110);
        let t113 = 1.0 + t110;
        let t114 = 1.0 / t113;
        let t115 = t112 * t114;
        let t117 = t105 * t105;
        let t118 = t108 * t108;
        let t119 = 1.0 / t118;
        let t122 = f64::exp(-0.150903e-2 * t117 * t119);
        let t123 = 1.0 - t122;
        let t124 = 1.0 / t105;
        let t127 = 10.0 / 81.0 * t124 * t108 - 1.0;
        let t129 = t110 * t115 + t123 * t127 + 1.0;
        let t133 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t129);
        let tzk0 = 2.0 * t133;
        zk[ip] += tzk0;
        let t135 = t18 / t52;
        let t139 = 1.0 / t88;
        let t140 = t22 * t139;
        let t143 = t21 * t21;
        let t144 = t143 * sigma[ip];
        let t145 = t88 * t23;
        let t146 = 1.0 / t145;
        let t147 = t144 * t146;
        let t148 = t27 * t27;
        let t149 = t148 * tau[ip];
        let t150 = 1.0 / t149;
        let t152 = 1.0 / t36 / t35;
        let t153 = t150 * t152;
        let t157 = (-0.1259478515625e-1 * t140 * t38 + 0.262391357421875e-3 * t147 * t153) * t42;
        let t158 = t157 * t47;
        let t162 = 1.0 / t52 / t24;
        let t163 = t51 * t162;
        let t169 = -5.0 / 3.0 * t58 * t54 + t163 / 3.0;
        let t170 = t169 * t42;
        let t171 = t47 * t73;
        let t175 = 1.0 / t72 / t71;
        let t176 = t67 * t175;
        let t179 = t63 * t83;
        let t180 = t85 * t169;
        let t183 = 0.22222222222222222222e0 * t170 * t68 + 0.12345679012345679012e0 * t179 * t180;
        let t186 = t76 * t163;
        let t188 = t170 * t171 / 4.0 - 9.0 / 40.0 * t176 * t183 - 2.0 / 27.0 * t186;
        let t193 = 1.0 / t96;
        let t194 = t79 * t193;
        let t195 = t21 * t25;
        let t196 = t195 * t32;
        let t199 = 1.0 / t19 / t145;
        let t201 = t86 * t87 * t199;
        let t203 = -324.0 * t196 - 1600.0 / 3.0 * t201;
        let t208 = t101 * rho[ip];
        let t209 = 1.0 / t208;
        let t212 = t158 * t55 / 24.0 - t48 * t163 / 9.0 + 292.0 / 2025.0 * t79 * t188 - 73.0 / 97200.0 * t188 * t96 - 73.0 / 194400.0 * t194 * t203 - 0.28272997284740921381e-3 * t201 - 0.39155829864091490256e-2 * t196 - 0.34976863408878213341e-4 * t22 * t209;
        let t213 = t212 * t109;
        let t215 = t108 * t107;
        let t216 = 1.0 / t215;
        let t217 = t105 * t216;
        let t218 = t217 * t115;
        let t222 = t217 * t42;
        let t223 = t47 * sigma[ip];
        let t224 = t50 * t162;
        let t225 = t223 * t224;
        let t226 = t222 * t225;
        let t228 = -0.1863e-3 * t213 - 0.58357848829441957076e-4 * t226;
        let t229 = t228 * t112;
        let t230 = t229 * t114;
        let t232 = t113 * t113;
        let t233 = 1.0 / t232;
        let t234 = t112 * t233;
        let t236 = t213 + 0.31324663891273192204e0 * t226;
        let t237 = t234 * t236;
        let t239 = t105 * t119;
        let t242 = t118 * t107;
        let t243 = 1.0 / t242;
        let t244 = t117 * t243;
        let t245 = t244 * t42;
        let t248 = -0.301806e-2 * t239 * t212 - 0.94539715103695970463e-3 * t245 * t225;
        let t249 = t248 * t122;
        let t250 = t249 * t127;
        let t251 = 1.0 / t117;
        let t252 = t251 * t108;
        let t255 = t124 * t107;
        let t256 = t255 * t42;
        let t259 = -10.0 / 81.0 * t252 * t212 - 0.38672424557127397783e-1 * t256 * t225;
        let t261 = t213 * t115 + 0.31324663891273192204e0 * t218 * t186 + t110 * t230 - t110 * t237 - t250 + t123 * t259;
        let t266 = piecewise3(t3, 0.0, -t7 * t135 * t129 / 8.0 - 3.0 / 8.0 * t7 * t20 * t261);
        let tvrho0 = 2.0 * rho[ip] * t266 + 2.0 * t133;
        vrho[ip] += tvrho0;
        let t271 = 1.0 / t89;
        let t272 = t143 * t271;
        let t276 = (0.1259478515625e-1 * t195 * t38 - 0.262391357421875e-3 * t272 * t153) * t42;
        let t277 = t276 * t47;
        let t280 = t47 * t50;
        let t281 = t280 * t54;
        let t284 = t50 * t54;
        let t285 = t76 * t73;
        let t286 = t284 * t285;
        let t288 = t76 * t67;
        let t289 = t284 * t288;
        let t291 = t85 * t50;
        let t293 = t179 * t291 * t54;
        let t295 = -0.27777777777777777778e-1 * t289 - 0.15432098765432098765e-1 * t293;
        let t298 = t284 * t76;
        let t300 = -t286 / 32.0 - 9.0 / 40.0 * t176 * t295 + t298 / 36.0;
        let t305 = sigma[ip] * t30;
        let t306 = t305 * t32;
        let t308 = sigma[ip] * t49;
        let t310 = t86 * t308 * t91;
        let t312 = 324.0 * t306 + 200.0 * t310;
        let t319 = t277 * t55 / 24.0 + t43 * t281 / 24.0 + 292.0 / 2025.0 * t79 * t300 - 73.0 / 97200.0 * t300 * t96 - 73.0 / 194400.0 * t194 * t312 + 0.10602373981777845518e-3 * t310 + 0.39155829864091490256e-2 * t306 + 0.13116323778329330003e-4 * t21 * t102;
        let t320 = t319 * t109;
        let t325 = t217 * t50;
        let t326 = t54 * t42;
        let t327 = t326 * t47;
        let t328 = t325 * t327;
        let t330 = -0.1863e-3 * t320 + 0.21884193311040733904e-4 * t328;
        let t331 = t330 * t112;
        let t332 = t331 * t114;
        let t335 = t320 - 0.11746748959227447077e0 * t328;
        let t336 = t234 * t335;
        let t340 = t244 * t50;
        let t343 = -0.301806e-2 * t239 * t319 + 0.35452393163885988924e-3 * t340 * t327;
        let t344 = t343 * t122;
        let t345 = t344 * t127;
        let t348 = t255 * t50;
        let t351 = -10.0 / 81.0 * t252 * t319 + 0.14502159208922774169e-1 * t348 * t327;
        let t353 = t320 * t115 - 0.11746748959227447077e0 * t218 * t298 + t110 * t332 - t110 * t336 - t345 + t123 * t351;
        let t357 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t353);
        let tvsigma0 = 2.0 * rho[ip] * t357;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t359 = 1.0 / t148;
        let t360 = t359 * t37;
        let t363 = t144 * t271;
        let t364 = t148 * t27;
        let t365 = 1.0 / t364;
        let t366 = t365 * t152;
        let t370 = (-0.1259478515625e-1 * t26 * t360 + 0.262391357421875e-3 * t363 * t366) * t42;
        let t371 = t370 * t47;
        let t374 = t50 * t60;
        let t382 = 0.22222222222222222222e0 * t374 * t288 + 0.12345679012345679012e0 * t179 * t291 * t60;
        let t385 = t374 * t285 / 4.0 - 9.0 / 40.0 * t176 * t382;
        let t390 = t31 * t29;
        let t394 = t371 * t55 / 24.0 + 292.0 / 2025.0 * t79 * t385 - 73.0 / 97200.0 * t385 * t96 + 73.0 / 600.0 * t194 * t390 - 0.39155829864091490256e-2 * t390;
        let t395 = t394 * t109;
        let t397 = t394 * t112;
        let t398 = t397 * t114;
        let t401 = t234 * t394;
        let t403 = t394 * t122;
        let t404 = t403 * t127;
        let t407 = t123 * t251;
        let t408 = t108 * t394;
        let t411 = t395 * t115 - 0.1863e-3 * t239 * t398 - t239 * t401 + 0.301806e-2 * t239 * t404 - 10.0 / 81.0 * t407 * t408;
        let t415 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t411);
        let tvtau0 = 2.0 * rho[ip] * t415;
        vtau[ip] += tvtau0;
    }
}
