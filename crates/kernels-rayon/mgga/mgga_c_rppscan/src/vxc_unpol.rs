//! MGGA_C_RPPSCAN vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rppscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rppscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(t34, t35 * zeta_threshold, 1.0);
        let t39 = 2.0 * t37 - 2.0;
        let t40 = M_CBRT2;
        let t41 = t40 - 1.0;
        let t43 = 1.0 / t41 / 2.0;
        let t44 = t39 * t43;
        let t46 = 1.0 + 0.0278125 * t11;
        let t51 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let t59 = f64::ln(2.0);
        let t60 = 1.0 - t59;
        let t61 = M_PI * M_PI;
        let t63 = t60 / t61;
        let t64 = t35 * t35;
        let t65 = piecewise3(t34, t64, 1.0);
        let t66 = t65 * t65;
        let t67 = t66 * t65;
        let t69 = 1.0 + 0.025 * t11;
        let t71 = 1.0 + 0.04445 * t11;
        let t72 = 1.0 / t71;
        let t73 = t69 * t72;
        let t74 = 1.0 / t60;
        let t77 = 1.0 / t67;
        let t78 = t61 * t77;
        let t80 = f64::exp(-(-t33 + t58) * t74 * t78);
        let t81 = t80 - 1.0;
        let t82 = 1.0 / t81;
        let t83 = t74 * t82;
        let t84 = t83 * sigma[ip];
        let t85 = t73 * t84;
        let t86 = rho[ip] * rho[ip];
        let t88 = 1.0 / t8 / t86;
        let t89 = t88 * t40;
        let t90 = 1.0 / t66;
        let t92 = 1.0 / t4;
        let t94 = t19 * t92 * t6;
        let t98 = 1.0 + 0.027439371595564633 * t85 * t89 * t90 * t94;
        let t99 = pow_1_4(t98);
        let t101 = 1.0 - 1.0 / t99;
        let t104 = 1.0 + 1.0 * t101 * t81;
        let t105 = f64::ln(t104);
        let t107 = t63 * t67 * t105;
        let t109 = 1.0 / t22 / rho[ip];
        let t112 = 1.0 / t22 / t86;
        let t115 = tau[ip] * t109 - sigma[ip] * t112 / 8.0;
        let t116 = M_CBRT6;
        let t117 = t116 * t116;
        let t118 = pow_1_3(t61);
        let t119 = t118 * t118;
        let t123 = param_eta * sigma[ip];
        let t126 = 3.0 / 20.0 * t117 * t119 * t40 + t123 * t112 / 8.0;
        let t127 = 1.0 / t126;
        let t128 = t115 * t127;
        let t129 = t128 <= 2.5;
        let t130 = 2.5 < t128;
        let t131 = piecewise3(t130, 2.5, t128);
        let t133 = t131 * t131;
        let t135 = t133 * t131;
        let t137 = t133 * t133;
        let t139 = t137 * t131;
        let t141 = t137 * t133;
        let t146 = piecewise3(t130, t128, 2.5);
        let t147 = 1.0 - t146;
        let t150 = f64::exp(1.5 / t147);
        let t152 = piecewise3(t129, 1.0 - 0.64 * t131 - 0.4352 * t133 - 1.535685604549 * t135 + 3.061560252175 * t137 - 1.915710236206 * t139 + 0.516884468372 * t141 - 0.051848879792 * t137 * t135, -0.7 * t150);
        let t155 = 1.0 + 0.04445 * t14 + 0.03138525 * t11;
        let t156 = 1.0 / t155;
        let t159 = f64::exp(1.0 * t156);
        let t160 = t159 - 1.0;
        let t161 = 1.0 / t119;
        let t162 = t116 * t161;
        let t163 = t40 * t40;
        let t164 = t163 * sigma[ip];
        let t168 = 1.0 + 0.02133764210437636 * t162 * t164 * t112;
        let t169 = pow_1_4(t168);
        let t171 = 1.0 - 1.0 / t169;
        let t173 = t160 * t171 + 1.0;
        let t174 = f64::ln(t173);
        let t180 = 1.0 - 2.363 * t41 * t39 * t43;
        let t182 = (-0.0285764 * t156 + 0.0285764 * t174) * t180 + t33 - t58 - t107;
        let t183 = t152 * t182;
        let tzk0 = -t33 + t58 + t107 + t183;
        zk[ip] += tzk0;
        let t185 = 1.0 / t8 / rho[ip];
        let t186 = t7 * t185;
        let t188 = t5 * t186 * t31;
        let t189 = 0.0011073470983333333 * t188;
        let t190 = t27 * t27;
        let t191 = 1.0 / t190;
        let t192 = t13 * t191;
        let t194 = 1.0 / t14 * t2;
        let t195 = t4 * t7;
        let t196 = t195 * t185;
        let t197 = t194 * t196;
        let t199 = t5 * t186;
        let t201 = f64::sqrt(t11);
        let t202 = t201 * t2;
        let t203 = t202 * t196;
        let t206 = t21 * t6 * t109;
        let t208 = -0.632975 * t197 - 0.29896666666666666 * t199 - 0.1023875 * t203 - 0.08215666666666667 * t206;
        let t209 = 1.0 / t30;
        let t210 = t208 * t209;
        let t211 = t192 * t210;
        let t212 = 1.0 * t211;
        let t213 = t44 * t2;
        let t216 = t213 * t195 * t185 * t55;
        let t217 = 0.00018311447306006544 * t216;
        let t218 = t44 * t46;
        let t219 = t51 * t51;
        let t220 = 1.0 / t219;
        let t225 = -0.8630833333333333 * t197 - 0.301925 * t199 - 0.05501625 * t203 - 0.082785 * t206;
        let t227 = 1.0 / t54;
        let t228 = t220 * t225 * t227;
        let t229 = t218 * t228;
        let t230 = 0.5848223622634646 * t229;
        let t232 = 1.0 / t99 / t98;
        let t233 = t86 * rho[ip];
        let t235 = 1.0 / t22 / t233;
        let t236 = t235 * t72;
        let t239 = t40 * t90;
        let t240 = t82 * sigma[ip] * t239;
        let t243 = t71 * t71;
        let t244 = 1.0 / t243;
        let t245 = t69 * t244;
        let t246 = t245 * t83;
        let t247 = sigma[ip] * t235;
        let t251 = t60 * t60;
        let t252 = 1.0 / t251;
        let t253 = t73 * t252;
        let t254 = t81 * t81;
        let t255 = 1.0 / t254;
        let t256 = t255 * sigma[ip];
        let t257 = t256 * t89;
        let t258 = t253 * t257;
        let t259 = t66 * t66;
        let t261 = 1.0 / t259 / t65;
        let t262 = t261 * t19;
        let t263 = t262 * t92;
        let t264 = t189 + t212 - t217 - t230;
        let t266 = t61 * t80;
        let t267 = t6 * t264 * t266;
        let t268 = t263 * t267;
        let t272 = 1.0 / t8 / t233;
        let t273 = t272 * t40;
        let t278 = -0.002743937159556463 * t236 * t74 * t240 + 0.004878720269691391 * t246 * t247 * t239 + 0.027439371595564633 * t258 * t268 - 0.0640252003896508 * t85 * t273 * t90 * t94;
        let t279 = t232 * t278;
        let t284 = t78 * t80;
        let t287 = 0.25 * t279 * t81 - 1.0 * t101 * t264 * t74 * t284;
        let t289 = 1.0 / t104;
        let t291 = t63 * t67 * t287 * t289;
        let t295 = -5.0 / 3.0 * tau[ip] * t112 + t247 / 3.0;
        let t297 = t126 * t126;
        let t298 = 1.0 / t297;
        let t299 = t115 * t298;
        let t300 = t123 * t235;
        let t303 = t295 * t127 + t299 * t300 / 3.0;
        let t304 = piecewise3(t130, 0.0, t303);
        let t306 = t131 * t304;
        let t308 = t133 * t304;
        let t310 = t135 * t304;
        let t312 = t137 * t304;
        let t314 = t139 * t304;
        let t319 = t147 * t147;
        let t320 = 1.0 / t319;
        let t321 = piecewise3(t130, t303, 0.0);
        let t325 = piecewise3(t129, -0.64 * t304 - 0.8704 * t306 - 4.607056813647 * t308 + 12.2462410087 * t310 - 9.57855118103 * t312 + 3.101306810232 * t314 - 0.362942158544 * t141 * t304, -1.05 * t320 * t321 * t150);
        let t326 = t325 * t182;
        let t327 = t155 * t155;
        let t328 = 1.0 / t327;
        let t331 = -0.007408333333333334 * t197 - 0.01046175 * t199;
        let t332 = t328 * t331;
        let t334 = t159 * t171;
        let t338 = 1.0 / t169 / t168;
        let t339 = t160 * t338;
        let t340 = t339 * t116;
        let t341 = t161 * t163;
        let t345 = -1.0 * t332 * t334 - 0.014225094736250906 * t340 * t341 * t247;
        let t346 = 1.0 / t173;
        let t351 = (0.0285764 * t332 + 0.0285764 * t345 * t346) * t180 - t189 - t212 + t217 + t230 - t291;
        let t352 = t152 * t351;
        let tvrho0 = -t33 + t58 + t107 + t183 + rho[ip] * (t189 + t212 - t217 - t230 + t291 + t326 + t352);
        vrho[ip] += tvrho0;
        let t355 = t65 * t232;
        let t356 = t73 * t88;
        let t357 = t355 * t356;
        let t358 = t40 * t19;
        let t359 = t92 * t6;
        let t360 = t359 * t289;
        let t361 = t358 * t360;
        let t363 = 0.0006950474021161377 * t357 * t361;
        let t364 = t112 * t127;
        let t365 = param_eta * t112;
        let t368 = -t299 * t365 / 8.0 - t364 / 8.0;
        let t369 = piecewise3(t130, 0.0, t368);
        let t371 = t131 * t369;
        let t373 = t133 * t369;
        let t375 = t135 * t369;
        let t377 = t137 * t369;
        let t379 = t139 * t369;
        let t384 = piecewise3(t130, t368, 0.0);
        let t388 = piecewise3(t129, -0.64 * t369 - 0.8704 * t371 - 4.607056813647 * t373 + 12.2462410087 * t375 - 9.57855118103 * t377 + 3.101306810232 * t379 - 0.362942158544 * t141 * t369, -1.05 * t320 * t384 * t150);
        let t389 = t388 * t182;
        let t390 = t339 * t162;
        let t391 = t163 * t112;
        let t392 = t346 * t180;
        let t396 = 0.00015243824895787514 * t390 * t391 * t392 - t363;
        let t397 = t152 * t396;
        let tvsigma0 = rho[ip] * (t363 + t389 + t397);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t399 = t109 * t127;
        let t400 = piecewise3(t130, 0.0, t399);
        let t402 = t131 * t400;
        let t404 = t133 * t400;
        let t406 = t135 * t400;
        let t408 = t137 * t400;
        let t410 = t139 * t400;
        let t415 = piecewise3(t130, t399, 0.0);
        let t419 = piecewise3(t129, -0.64 * t400 - 0.8704 * t402 - 4.607056813647 * t404 + 12.2462410087 * t406 - 9.57855118103 * t408 + 3.101306810232 * t410 - 0.362942158544 * t141 * t400, -1.05 * t320 * t415 * t150);
        let t420 = rho[ip] * t419;
        let tvtau0 = t420 * t182;
        vtau[ip] += tvtau0;
    }
}
