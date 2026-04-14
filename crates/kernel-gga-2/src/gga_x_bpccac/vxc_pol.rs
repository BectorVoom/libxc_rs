//! GGA_X_BPCCAC vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 97 shared lines across all orders.
//! Delta: 113 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_bpccac_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (97 lines) ---
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
        let t27 = t25 * t26;
        let t28 = f64::sqrt(sigma0);
        let t29 = pow_1_3(rho0);
        let t31 = 1.0 / t29 / rho0;
        let t32 = t28 * t31;
        let t34 = f64::exp(-t32 + 19.0);
        let t35 = 1.0 + t34;
        let t36 = 1.0 / t35;
        let t37 = 1.0 - t36;
        let t38 = M_CBRT6;
        let t39 = M_PI * M_PI;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t38 * t42;
        let t44 = rho0 * rho0;
        let t45 = t29 * t29;
        let t47 = 1.0 / t45 / t44;
        let t49 = t43 * sigma0 * t47;
        let t51 = 0.1227e1 + 0.91464571985215458336e-2 * t49;
        let t54 = 0.2227e1 - 0.1505529e1 / t51;
        let t57 = f64::exp(-25.0 / 6.0 * t49);
        let t60 = (0.2743e0 - 0.1508e0 * t57) * t38;
        let t61 = t42 * sigma0;
        let t65 = t38 * t38;
        let t67 = 1.0 / t40 / t39;
        let t68 = t65 * t67;
        let t69 = sigma0 * sigma0;
        let t70 = t44 * t44;
        let t71 = t70 * rho0;
        let t73 = 1.0 / t29 / t71;
        let t76 = 0.69444444444444444444e-5 * t68 * t69 * t73;
        let t77 = t60 * t61 * t47 / 24.0 - t76;
        let t79 = t65 / t40;
        let t82 = f64::ln(0.64963333333333333333e0 * t79 * t32 + f64::sqrt(pow_2(0.64963333333333333333e0 * t79 * t32) + 1.0));
        let t86 = 1.0 + 0.16370833333333333333e-1 * t79 * t32 * t82 + t76;
        let t87 = 1.0 / t86;
        let t89 = t77 * t87 + 1.0;
        let t91 = t36 * t89 + t37 * t54;
        let t95 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t91);
        let t96 = rho1 <= dens_threshold;
        let t97 = -t16;
        let t99 = piecewise5(t14, t11, t10, t15, t97 * t7);
        let t100 = 1.0 + t99;
        let t101 = t100 <= zeta_threshold;
        let t102 = pow_1_3(t100);
        let t104 = piecewise3(t101, t22, t102 * t100);
        let t105 = t104 * t26;
        let t106 = f64::sqrt(sigma2);
        let t107 = pow_1_3(rho1);
        let t109 = 1.0 / t107 / rho1;
        let t110 = t106 * t109;
        let t112 = f64::exp(-t110 + 19.0);
        let t113 = 1.0 + t112;
        let t114 = 1.0 / t113;
        let t115 = 1.0 - t114;
        let t116 = rho1 * rho1;
        let t117 = t107 * t107;
        let t119 = 1.0 / t117 / t116;
        let t121 = t43 * sigma2 * t119;
        let t123 = 0.1227e1 + 0.91464571985215458336e-2 * t121;
        let t126 = 0.2227e1 - 0.1505529e1 / t123;
        let t129 = f64::exp(-25.0 / 6.0 * t121);
        let t132 = (0.2743e0 - 0.1508e0 * t129) * t38;
        let t133 = t42 * sigma2;
        let t137 = sigma2 * sigma2;
        let t138 = t116 * t116;
        let t139 = t138 * rho1;
        let t141 = 1.0 / t107 / t139;
        let t144 = 0.69444444444444444444e-5 * t68 * t137 * t141;
        let t145 = t132 * t133 * t119 / 24.0 - t144;
        let t148 = f64::ln(0.64963333333333333333e0 * t79 * t110 + f64::sqrt(pow_2(0.64963333333333333333e0 * t79 * t110) + 1.0));
        let t152 = 1.0 + 0.16370833333333333333e-1 * t79 * t110 * t148 + t144;
        let t153 = 1.0 / t152;
        let t155 = t145 * t153 + 1.0;
        let t157 = t114 * t155 + t115 * t126;
        let t161 = piecewise3(t96, 0.0, -3.0 / 8.0 * t5 * t105 * t157);
        let tzk0 = t95 + t161;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (113 lines) ---
        let t162 = t6 * t6;
        let t163 = 1.0 / t162;
        let t164 = t16 * t163;
        let t166 = piecewise5(t10, 0.0, t14, 0.0, t7 - t164);
        let t169 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t166);
        let t170 = t169 * t26;
        let t174 = t26 * t26;
        let t175 = 1.0 / t174;
        let t176 = t25 * t175;
        let t179 = t5 * t176 * t91 / 8.0;
        let t180 = t35 * t35;
        let t181 = 1.0 / t180;
        let t182 = t181 * t28;
        let t184 = 1.0 / t29 / t44;
        let t186 = t184 * t34 * t54;
        let t189 = t51 * t51;
        let t190 = 1.0 / t189;
        let t191 = t37 * t190;
        let t192 = t191 * t38;
        let t193 = t44 * rho0;
        let t195 = 1.0 / t45 / t193;
        let t196 = t61 * t195;
        let t199 = t181 * t89;
        let t200 = t28 * t184;
        let t201 = t200 * t34;
        let t204 = t70 * t44;
        let t206 = 1.0 / t29 / t204;
        let t207 = t69 * t206;
        let t214 = 0.37037037037037037037e-4 * t68 * t207;
        let t215 = -0.69814814814814814817e-1 * t68 * t207 * t57 - t60 * t196 / 9.0 + t214;
        let t217 = t86 * t86;
        let t218 = 1.0 / t217;
        let t219 = t77 * t218;
        let t225 = 0.25321408066666666666e1 * t49 + 1.0;
        let t226 = f64::sqrt(t225);
        let t227 = 1.0 / t226;
        let t231 = -0.21827777777777777777e-1 * t79 * t200 * t82 - 0.85080312222222222219e-1 * t43 * sigma0 * t195 * t227 - t214;
        let t233 = t215 * t87 - t219 * t231;
        let t235 = 4.0 / 3.0 * t182 * t186 - 0.36720684159021185007e-1 * t192 * t196 - 4.0 / 3.0 * t199 * t201 + t36 * t233;
        let t240 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t170 * t91 - t179 - 3.0 / 8.0 * t5 * t27 * t235);
        let t241 = t97 * t163;
        let t243 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t241);
        let t246 = piecewise3(t101, 0.0, 4.0 / 3.0 * t102 * t243);
        let t247 = t246 * t26;
        let t251 = t104 * t175;
        let t254 = t5 * t251 * t157 / 8.0;
        let t256 = piecewise3(t96, 0.0, -3.0 / 8.0 * t5 * t247 * t157 - t254);
        let tvrho0 = t95 + t161 + t6 * (t240 + t256);
        vrho[ip * 2] += tvrho0;
        let t260 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t164);
        let t263 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t260);
        let t264 = t263 * t26;
        let t269 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t264 * t91 - t179);
        let t271 = piecewise5(t14, 0.0, t10, 0.0, t7 - t241);
        let t274 = piecewise3(t101, 0.0, 4.0 / 3.0 * t102 * t271);
        let t275 = t274 * t26;
        let t279 = t113 * t113;
        let t280 = 1.0 / t279;
        let t281 = t280 * t106;
        let t283 = 1.0 / t107 / t116;
        let t285 = t283 * t112 * t126;
        let t288 = t123 * t123;
        let t289 = 1.0 / t288;
        let t290 = t115 * t289;
        let t291 = t290 * t38;
        let t292 = t116 * rho1;
        let t294 = 1.0 / t117 / t292;
        let t295 = t133 * t294;
        let t298 = t280 * t155;
        let t299 = t106 * t283;
        let t300 = t299 * t112;
        let t303 = t138 * t116;
        let t305 = 1.0 / t107 / t303;
        let t306 = t137 * t305;
        let t313 = 0.37037037037037037037e-4 * t68 * t306;
        let t314 = -0.69814814814814814817e-1 * t68 * t306 * t129 - t132 * t295 / 9.0 + t313;
        let t316 = t152 * t152;
        let t317 = 1.0 / t316;
        let t318 = t145 * t317;
        let t324 = 0.25321408066666666666e1 * t121 + 1.0;
        let t325 = f64::sqrt(t324);
        let t326 = 1.0 / t325;
        let t330 = -0.21827777777777777777e-1 * t79 * t299 * t148 - 0.85080312222222222219e-1 * t43 * sigma2 * t294 * t326 - t313;
        let t332 = t314 * t153 - t318 * t330;
        let t334 = 4.0 / 3.0 * t281 * t285 - 0.36720684159021185007e-1 * t291 * t295 - 4.0 / 3.0 * t298 * t300 + t114 * t332;
        let t339 = piecewise3(t96, 0.0, -3.0 / 8.0 * t5 * t275 * t157 - t254 - 3.0 / 8.0 * t5 * t105 * t334);
        let tvrho1 = t95 + t161 + t6 * (t269 + t339);
        vrho[ip * 2 + 1] += tvrho1;
        let t342 = 1.0 / t28;
        let t343 = t181 * t342;
        let t345 = t31 * t34 * t54;
        let t351 = t342 * t31;
        let t352 = t351 * t34;
        let t355 = t73 * t57;
        let t364 = 0.13888888888888888889e-4 * t68 * sigma0 * t73;
        let t365 = 0.26180555555555555555e-1 * t68 * t355 * sigma0 + t60 * t42 * t47 / 24.0 - t364;
        let t373 = 0.81854166666666666665e-2 * t79 * t351 * t82 + 0.31905117083333333333e-1 * t43 * t47 * t227 + t364;
        let t375 = -t219 * t373 + t365 * t87;
        let t377 = -t343 * t345 / 2.0 + 0.13770256559632944377e-1 * t191 * t43 * t47 + t199 * t352 / 2.0 + t36 * t375;
        let t381 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t377);
        let tvsigma0 = t6 * t381;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t382 = 1.0 / t106;
        let t383 = t280 * t382;
        let t385 = t109 * t112 * t126;
        let t391 = t382 * t109;
        let t392 = t391 * t112;
        let t395 = t141 * t129;
        let t404 = 0.13888888888888888889e-4 * t68 * sigma2 * t141;
        let t405 = 0.26180555555555555555e-1 * t68 * t395 * sigma2 + t132 * t42 * t119 / 24.0 - t404;
        let t413 = 0.81854166666666666665e-2 * t79 * t391 * t148 + 0.31905117083333333333e-1 * t43 * t119 * t326 + t404;
        let t415 = t405 * t153 - t318 * t413;
        let t417 = -t383 * t385 / 2.0 + 0.13770256559632944377e-1 * t290 * t43 * t119 + t298 * t392 / 2.0 + t114 * t415;
        let t421 = piecewise3(t96, 0.0, -3.0 / 8.0 * t5 * t105 * t417);
        let tvsigma2 = t6 * t421;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
