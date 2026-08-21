//! MGGA_X_GVT4 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gvt4_fxc_unpol(
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
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t22 = sigma[ip] * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t29 = tau[ip] * t21;
        let t31 = 1.0 / t24 / rho[ip];
        let t32 = t29 * t31;
        let t34 = M_CBRT6;
        let t35 = t34 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = t35 * t38;
        let t41 = 1.0 + 0.00186726 * t27 + 0.00373452 * t32 - 0.001120356 * t39;
        let t47 = -0.003556788 * t27 + 0.012500652 * t32 - 0.0037501956 * t39;
        let t48 = t41 * t41;
        let t49 = 1.0 / t48;
        let t51 = sigma[ip] * sigma[ip];
        let t52 = t51 * t20;
        let t53 = t23 * t23;
        let t54 = t53 * rho[ip];
        let t56 = 1.0 / t18 / t54;
        let t61 = 2.0 * t32 - 3.0 / 5.0 * t39;
        let t65 = t61 * t61;
        let t67 = -4.709036e-05 * t52 * t56 - 0.0001282732 * t22 * t26 * t61 + 0.0003574822 * t65;
        let t68 = t48 * t41;
        let t69 = 1.0 / t68;
        let t73 = pow_1_3(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3(t3, 0.0, t19 * t77 / 4.0);
        let tzk0 = 2.0 * t80;
        zk[ip] += tzk0;
        let t82 = t17 / t24;
        let t85 = t23 * rho[ip];
        let t87 = 1.0 / t24 / t85;
        let t88 = t22 * t87;
        let t90 = t29 * t26;
        let t92 = -0.00497936 * t88 - 0.0062242 * t90;
        let t97 = 0.009484768 * t88 - 0.02083442 * t90;
        let t99 = t47 * t69;
        let t102 = t53 * t23;
        let t104 = 1.0 / t18 / t102;
        let t110 = sigma[ip] * t20;
        let t114 = t61 * tau[ip];
        let t115 = t21 * t26;
        let t118 = 0.00025114858666666666 * t52 * t104 + 0.00034206186666666666 * t22 * t87 * t61 + 0.0008551546666666666 * t110 * t56 * tau[ip] - 0.0023832146666666666 * t114 * t115;
        let t120 = t48 * t48;
        let t121 = 1.0 / t120;
        let t122 = t67 * t121;
        let t127 = (0.9800683 * t49 * t92 + t97 * t49 - 2.0 * t99 * t92 + t118 * t69 - 3.0 * t122 * t92) * t74 * t76;
        let t131 = piecewise3(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t80;
        vrho[ip] += tvrho0;
        let t134 = t49 * t21;
        let t135 = t134 * t26;
        let t137 = t99 * t115;
        let t139 = t110 * t56;
        let t141 = t115 * t61;
        let t143 = -9.418072e-05 * t139 - 0.0001282732 * t141;
        let t145 = t122 * t115;
        let t149 = (-0.001726745666142 * t135 - 0.00373452 * t137 + t143 * t69 - 0.00560178 * t145) * t74 * t76;
        let t152 = piecewise3(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.0005130928 * t110 * t160 + 0.0014299288 * t61 * t21 * t31;
        let t172 = (0.016160736667716 * t134 * t31 - 0.00746904 * t99 * t156 + t166 * t69 - 0.01120356 * t122 * t156) * t74 * t76;
        let t175 = piecewise3(t3, 0.0, t19 * t172 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t175;
        vtau[ip] += tvtau0;
        let t178 = t17 * t31;
        let t183 = t92 * t92;
        let t187 = 1.0 / t24 / t53;
        let t188 = t22 * t187;
        let t190 = t29 * t87;
        let t192 = 0.018257653333333332 * t188 + 0.016597866666666666 * t190;
        let t197 = -0.034777482666666665 * t188 + 0.055558453333333334 * t190;
        let t199 = t97 * t69;
        let t202 = t47 * t121;
        let t207 = t53 * t85;
        let t209 = 1.0 / t18 / t207;
        let t218 = tau[ip] * tau[ip];
        let t219 = t218 * t20;
        let t222 = t21 * t87;
        let t225 = -0.0015906077155555555 * t52 * t209 - 0.0012542268444444445 * t22 * t187 * t61 - 0.006841237333333333 * t110 * t104 * tau[ip] + 0.015888097777777777 * t219 * t56 + 0.006355239111111111 * t114 * t222;
        let t227 = t118 * t121;
        let t231 = 1.0 / t120 / t41;
        let t232 = t67 * t231;
        let t239 = (-1.9601366 * t69 * t183 + 0.9800683 * t49 * t192 + t197 * t49 - 4.0 * t199 * t92 + 6.0 * t202 * t183 - 2.0 * t99 * t192 + t225 * t69 - 6.0 * t227 * t92 + 12.0 * t232 * t183 - 3.0 * t122 * t192) * t74 * t76;
        let t243 = piecewise3(t3, 0.0, -t178 * t77 / 18.0 + t82 * t127 / 6.0 + t19 * t239 / 4.0);
        let tv2rho20 = 2.0 * rho[ip] * t243 + 4.0 * t131;
        v2rho2[ip] += tv2rho20;
        let t248 = t69 * t21;
        let t250 = t248 * t26 * t92;
        let t252 = t134 * t87;
        let t254 = t199 * t115;
        let t256 = t115 * t92;
        let t257 = t202 * t256;
        let t259 = t99 * t222;
        let t261 = t110 * t104;
        let t263 = t222 * t61;
        let t265 = t20 * t56;
        let t266 = t265 * tau[ip];
        let t268 = 0.0005022971733333333 * t261 + 0.00034206186666666666 * t263 + 0.0008551546666666666 * t266;
        let t270 = t143 * t121;
        let t273 = t227 * t115;
        let t275 = t232 * t256;
        let t277 = t122 * t222;
        let t281 = (0.003453491332284 * t250 + 0.004604655109712 * t252 - 0.00373452 * t254 + 0.01120356 * t257 + 0.00995872 * t259 + t268 * t69 - 3.0 * t270 * t92 - 0.00560178 * t273 + 0.02240712 * t275 + 0.01493808 * t277) * t74 * t76;
        let t285 = piecewise3(t3, 0.0, t82 * t149 / 12.0 + t19 * t281 / 4.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t285 + 2.0 * t152;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t290 = t31 * t92;
        let t296 = t156 * t92;
        let t305 = 0.0022234021333333333 * t139 - 0.009532858666666666 * tau[ip] * t20 * t160 - 0.0023832146666666666 * t141;
        let t307 = t166 * t121;
        let t317 = (-0.032321473335432 * t248 * t290 - 0.02693456111286 * t135 - 0.00746904 * t199 * t156 + 0.02240712 * t202 * t296 + 0.0124484 * t137 + t305 * t69 - 3.0 * t307 * t92 - 0.01120356 * t227 * t156 + 0.04481424 * t232 * t296 + 0.0186726 * t145) * t74 * t76;
        let t321 = piecewise3(t3, 0.0, t82 * t172 / 12.0 + t19 * t317 / 4.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t321 + 2.0 * t175;
        v2rhotau[ip] += tv2rhotau0;
        let t324 = t69 * t20;
        let t325 = t324 * t56;
        let t327 = t202 * t265;
        let t329 = t270 * t115;
        let t331 = t232 * t265;
        let t335 = (-5.471779570623876e-05 * t325 + 4.18399188912e-05 * t327 - 0.01120356 * t329 + 8.36798377824e-05 * t331) * t74 * t76;
        let t338 = piecewise3(t3, 0.0, t19 * t335 / 4.0);
        let tv2sigma20 = 2.0 * rho[ip] * t338;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t340 = t324 * t160;
        let t342 = t20 * t160;
        let t343 = t202 * t342;
        let t345 = t307 * t115;
        let t349 = t232 * t342;
        let t353 = (-0.0005806664049135975 * t340 + 8.36798377824e-05 * t343 - 0.00560178 * t345 - 0.01120356 * t270 * t156 + 0.0001673596755648 * t349) * t74 * t76;
        let t356 = piecewise3(t3, 0.0, t19 * t353 / 4.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t356;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t359 = 1.0 / t18 / t85;
        let t362 = t20 * t359;
        let t371 = (0.005291569083170565 * t324 * t359 + 0.0001673596755648 * t202 * t362 - 0.02240712 * t307 * t156 + 0.0003347193511296 * t232 * t362) * t74 * t76;
        let t374 = piecewise3(t3, 0.0, t19 * t371 / 4.0);
        let tv2tau20 = 2.0 * rho[ip] * t374;
        v2tau2[ip] += tv2tau20;
    }
}
