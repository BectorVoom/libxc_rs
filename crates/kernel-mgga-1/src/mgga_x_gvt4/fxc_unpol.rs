//! MGGA_X_GVT4 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 48 shared lines across all orders.
//! Delta: 72 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gvt4_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (48 lines) ---
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
        let t41 = 1.0 + 0.186726e-2 * t27 + 0.373452e-2 * t32 - 0.1120356e-2 * t39;
        let t47 = -0.3556788e-2 * t27 + 0.12500652e-1 * t32 - 0.37501956e-2 * t39;
        let t48 = t41 * t41;
        let t49 = 1.0 / t48;
        let t51 = sigma[ip] * sigma[ip];
        let t52 = t51 * t20;
        let t53 = t23 * t23;
        let t54 = t53 * rho[ip];
        let t56 = 1.0 / t18 / t54;
        let t61 = 2.0 * t32 - 3.0 / 5.0 * t39;
        let t65 = t61 * t61;
        let t67 = -0.4709036e-4 * t52 * t56 - 0.1282732e-3 * t22 * t26 * t61 + 0.3574822e-3 * t65;
        let t68 = t48 * t41;
        let t69 = 1.0 / t68;
        let t73 = pow_1_3(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683e0 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3(t3, 0.0, t19 * t77 / 4.0);
        let tzk0 = 2.0 * t80;
        zk[ip] += tzk0;
        // --- vxc delta (37 lines) ---
        let t82 = t17 / t24;
        let t85 = t23 * rho[ip];
        let t87 = 1.0 / t24 / t85;
        let t88 = t22 * t87;
        let t90 = t29 * t26;
        let t92 = -0.497936e-2 * t88 - 0.62242e-2 * t90;
        let t97 = 0.9484768e-2 * t88 - 0.2083442e-1 * t90;
        let t99 = t47 * t69;
        let t102 = t53 * t23;
        let t104 = 1.0 / t18 / t102;
        let t110 = sigma[ip] * t20;
        let t114 = t61 * tau[ip];
        let t115 = t21 * t26;
        let t118 = 0.25114858666666666667e-3 * t52 * t104 + 0.34206186666666666667e-3 * t22 * t87 * t61 + 0.85515466666666666667e-3 * t110 * t56 * tau[ip] - 0.23832146666666666667e-2 * t114 * t115;
        let t120 = t48 * t48;
        let t121 = 1.0 / t120;
        let t122 = t67 * t121;
        let t127 = (0.9800683e0 * t49 * t92 + t97 * t49 - 2.0 * t99 * t92 + t118 * t69 - 3.0 * t122 * t92) * t74 * t76;
        let t131 = piecewise3(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t80;
        vrho[ip] += tvrho0;
        let t134 = t49 * t21;
        let t135 = t134 * t26;
        let t137 = t99 * t115;
        let t139 = t110 * t56;
        let t141 = t115 * t61;
        let t143 = -0.9418072e-4 * t139 - 0.1282732e-3 * t141;
        let t145 = t122 * t115;
        let t149 = (-0.1726745666142e-2 * t135 - 0.373452e-2 * t137 + t143 * t69 - 0.560178e-2 * t145) * t74 * t76;
        let t152 = piecewise3(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.5130928e-3 * t110 * t160 + 0.14299288e-2 * t61 * t21 * t31;
        let t172 = (0.16160736667716e-1 * t134 * t31 - 0.746904e-2 * t99 * t156 + t166 * t69 - 0.1120356e-1 * t122 * t156) * t74 * t76;
        let t175 = piecewise3(t3, 0.0, t19 * t172 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t175;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (72 lines) ---
        let t178 = t17 * t31;
        let t183 = t92 * t92;
        let t187 = 1.0 / t24 / t53;
        let t188 = t22 * t187;
        let t190 = t29 * t87;
        let t192 = 0.18257653333333333333e-1 * t188 + 0.16597866666666666667e-1 * t190;
        let t197 = -0.34777482666666666667e-1 * t188 + 0.55558453333333333333e-1 * t190;
        let t199 = t97 * t69;
        let t202 = t47 * t121;
        let t207 = t53 * t85;
        let t209 = 1.0 / t18 / t207;
        let t218 = tau[ip] * tau[ip];
        let t219 = t218 * t20;
        let t222 = t21 * t87;
        let t225 = -0.15906077155555555556e-2 * t52 * t209 - 0.12542268444444444445e-2 * t22 * t187 * t61 - 0.68412373333333333334e-2 * t110 * t104 * tau[ip] + 0.15888097777777777778e-1 * t219 * t56 + 0.63552391111111111112e-2 * t114 * t222;
        let t227 = t118 * t121;
        let t231 = 1.0 / t120 / t41;
        let t232 = t67 * t231;
        let t239 = (-0.19601366e1 * t69 * t183 + 0.9800683e0 * t49 * t192 + t197 * t49 - 4.0 * t199 * t92 + 6.0 * t202 * t183 - 2.0 * t99 * t192 + t225 * t69 - 6.0 * t227 * t92 + 12.0 * t232 * t183 - 3.0 * t122 * t192) * t74 * t76;
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
        let t268 = 0.50229717333333333333e-3 * t261 + 0.34206186666666666667e-3 * t263 + 0.85515466666666666667e-3 * t266;
        let t270 = t143 * t121;
        let t273 = t227 * t115;
        let t275 = t232 * t256;
        let t277 = t122 * t222;
        let t281 = (0.3453491332284e-2 * t250 + 0.4604655109712e-2 * t252 - 0.373452e-2 * t254 + 0.1120356e-1 * t257 + 0.995872e-2 * t259 + t268 * t69 - 3.0 * t270 * t92 - 0.560178e-2 * t273 + 0.2240712e-1 * t275 + 0.1493808e-1 * t277) * t74 * t76;
        let t285 = piecewise3(t3, 0.0, t82 * t149 / 12.0 + t19 * t281 / 4.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t285 + 2.0 * t152;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t290 = t31 * t92;
        let t296 = t156 * t92;
        let t305 = 0.22234021333333333333e-2 * t139 - 0.95328586666666666667e-2 * tau[ip] * t20 * t160 - 0.23832146666666666667e-2 * t141;
        let t307 = t166 * t121;
        let t317 = (-0.32321473335432e-1 * t248 * t290 - 0.2693456111286e-1 * t135 - 0.746904e-2 * t199 * t156 + 0.2240712e-1 * t202 * t296 + 0.124484e-1 * t137 + t305 * t69 - 3.0 * t307 * t92 - 0.1120356e-1 * t227 * t156 + 0.4481424e-1 * t232 * t296 + 0.186726e-1 * t145) * t74 * t76;
        let t321 = piecewise3(t3, 0.0, t82 * t172 / 12.0 + t19 * t317 / 4.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t321 + 2.0 * t175;
        v2rhotau[ip] += tv2rhotau0;
        let t324 = t69 * t20;
        let t325 = t324 * t56;
        let t327 = t202 * t265;
        let t329 = t270 * t115;
        let t331 = t232 * t265;
        let t335 = (-0.5471779570623875632e-4 * t325 + 0.418399188912e-4 * t327 - 0.1120356e-1 * t329 + 0.836798377824e-4 * t331) * t74 * t76;
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
        let t353 = (-0.58066640491359751264e-3 * t340 + 0.836798377824e-4 * t343 - 0.560178e-2 * t345 - 0.1120356e-1 * t270 * t156 + 0.1673596755648e-3 * t349) * t74 * t76;
        let t356 = piecewise3(t3, 0.0, t19 * t353 / 4.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t356;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t359 = 1.0 / t18 / t85;
        let t362 = t20 * t359;
        let t371 = (0.52915690831705649747e-2 * t324 * t359 + 0.1673596755648e-3 * t202 * t362 - 0.2240712e-1 * t307 * t156 + 0.3347193511296e-3 * t232 * t362) * t74 * t76;
        let t374 = piecewise3(t3, 0.0, t19 * t371 / 4.0);
        let tv2tau20 = 2.0 * rho[ip] * t374;
        v2tau2[ip] += tv2tau20;
    }
}
