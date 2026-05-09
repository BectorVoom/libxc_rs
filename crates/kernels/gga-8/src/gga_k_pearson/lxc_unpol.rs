//! GGA_K_PEARSON lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 39 shared lines across all orders.
//! Delta: 15 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_pearson_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (39 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t29 = t24 / t27;
        let t30 = t29 * sigma[ip];
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = rho[ip] * rho[ip];
        let t37 = t25 * t25;
        let t38 = 1.0 / t37;
        let t39 = sigma[ip] * sigma[ip];
        let t40 = t39 * sigma[ip];
        let t41 = t38 * t40;
        let t42 = t33 * t33;
        let t43 = t42 * t42;
        let t47 = 1.0 + t41 / t43 / 576.0;
        let t48 = 1.0 / t47;
        let t49 = t32 / t22 / t33 * t48;
        let t52 = 1.0 + 5.0 / 648.0 * t30 * t49;
        let t56 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        // --- vxc delta (18 lines) ---
        let t58 = t20 / t21;
        let t62 = t33 * rho[ip];
        let t66 = t32 / t22 / t62 * t48;
        let t69 = t39 * t39;
        let t70 = t29 * t69;
        let t71 = t43 * t62;
        let t73 = 1.0 / t22 / t71;
        let t75 = t47 * t47;
        let t76 = 1.0 / t75;
        let t77 = t76 * t38;
        let t81 = -5.0 / 243.0 * t30 * t66 + 5.0 / 46656.0 * t70 * t32 * t73 * t77;
        let t86 = piecewise3(t2, 0.0, t7 * t58 * t52 / 10.0 + 3.0 / 20.0 * t7 * t23 * t81);
        let tvrho0 = 2.0 * rho[ip] * t86 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t92 = t43 * t33;
        let t94 = 1.0 / t22 / t92;
        let t99 = 5.0 / 648.0 * t29 * t49 - 5.0 / 124416.0 * t29 * t40 * t32 * t94 * t77;
        let t103 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (29 lines) ---
        let t108 = t20 / t21 / rho[ip];
        let t118 = t32 / t22 / t42 * t48;
        let t121 = t43 * t42;
        let t123 = 1.0 / t22 / t121;
        let t128 = t69 * t40;
        let t129 = t29 * t128;
        let t130 = t43 * t43;
        let t133 = 1.0 / t22 / t130 / t42;
        let t136 = 1.0 / t75 / t47;
        let t137 = t37 * t37;
        let t138 = 1.0 / t137;
        let t139 = t136 * t138;
        let t143 = 55.0 / 729.0 * t30 * t118 - 215.0 / 139968.0 * t70 * t32 * t123 * t77 + 5.0 / 1679616.0 * t129 * t32 * t133 * t139;
        let t148 = piecewise3(t2, 0.0, -t7 * t108 * t52 / 30.0 + t7 * t58 * t81 / 5.0 + 3.0 / 20.0 * t7 * t23 * t143);
        let tv2rho20 = 2.0 * rho[ip] * t148 + 4.0 * t86;
        v2rho2[ip] += tv2rho20;
        let t156 = t29 * t32;
        let t157 = t73 * t76;
        let t161 = t69 * t39;
        let t165 = 1.0 / t22 / t130 / t62;
        let t170 = -5.0 / 243.0 * t29 * t66 + 25.0 / 46656.0 * t156 * t157 * t41 - 5.0 / 4478976.0 * t29 * t161 * t32 * t165 * t139;
        let t175 = piecewise3(t2, 0.0, t7 * t58 * t99 / 10.0 + 3.0 / 20.0 * t7 * t23 * t170);
        let tv2rhosigma0 = 2.0 * rho[ip] * t175 + 2.0 * t103;
        v2rhosigma[ip] += tv2rhosigma0;
        let t178 = t94 * t76;
        let t179 = t38 * t39;
        let t183 = t69 * sigma[ip];
        let t187 = 1.0 / t22 / t130 / t33;
        let t192 = -5.0 / 31104.0 * t156 * t178 * t179 + 5.0 / 0.11943936e8 * t29 * t183 * t32 * t187 * t139;
        let t196 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t192);
        let tv2sigma20 = 2.0 * rho[ip] * t196;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (38 lines) ---
        let t201 = t20 / t21 / t33;
        let t211 = t42 * rho[ip];
        let t215 = t32 / t22 / t211 * t48;
        let t218 = t43 * t211;
        let t220 = 1.0 / t22 / t218;
        let t227 = 1.0 / t22 / t130 / t211;
        let t232 = t69 * t69;
        let t234 = t29 * t232 * t39;
        let t237 = 1.0 / t22 / t130 / t218;
        let t239 = t75 * t75;
        let t240 = 1.0 / t239;
        let t242 = 1.0 / t137 / t37;
        let t243 = t240 * t242;
        let t247 = -770.0 / 2187.0 * t30 * t215 + 1435.0 / 69984.0 * t70 * t32 * t220 * t77 - 175.0 / 1679616.0 * t129 * t32 * t227 * t139 + 5.0 / 0.40310784e8 * t234 * t32 * t237 * t243;
        let t252 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t201 * t52 - t7 * t108 * t81 / 10.0 + 3.0 / 10.0 * t7 * t58 * t143 + 3.0 / 20.0 * t7 * t23 * t247);
        let tv3rho30 = 2.0 * rho[ip] * t252 + 6.0 * t148;
        v3rho3[ip] += tv3rho30;
        let t264 = t123 * t76;
        let t268 = t133 * t136;
        let t269 = t138 * t161;
        let t273 = t232 * sigma[ip];
        let t274 = t29 * t273;
        let t277 = 1.0 / t22 / t130 / t121;
        let t282 = 55.0 / 729.0 * t29 * t118 - 305.0 / 46656.0 * t156 * t264 * t41 + 55.0 / 1492992.0 * t156 * t268 * t269 - 5.0 / 0.107495424e9 * t274 * t32 * t277 * t243;
        let t287 = piecewise3(t2, 0.0, -t7 * t108 * t99 / 30.0 + t7 * t58 * t170 / 5.0 + 3.0 / 20.0 * t7 * t23 * t282);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t287 + 4.0 * t175;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t296 = t165 * t136;
        let t297 = t138 * t183;
        let t304 = 1.0 / t22 / t130 / t71;
        let t309 = 5.0 / 2916.0 * t156 * t157 * t179 - 55.0 / 4478976.0 * t156 * t296 * t297 + 5.0 / 0.286654464e9 * t29 * t232 * t32 * t304 * t243;
        let t314 = piecewise3(t2, 0.0, t7 * t58 * t192 / 10.0 + 3.0 / 20.0 * t7 * t23 * t309);
        let tv3rhosigma20 = 2.0 * rho[ip] * t314 + 2.0 * t196;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t317 = t187 * t136;
        let t318 = t138 * t69;
        let t322 = t38 * sigma[ip];
        let t328 = 1.0 / t22 / t130 / t92;
        let t333 = 5.0 / 1327104.0 * t156 * t317 * t318 - 5.0 / 15552.0 * t156 * t178 * t322 - 5.0 / 0.764411904e9 * t129 * t32 * t328 * t243;
        let t337 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t333);
        let tv3sigma30 = 2.0 * rho[ip] * t337;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (15 lines) ---
        let t355 = t42 * t33;
        let t362 = t43 * t355;
        let t385 = t130 * t130;
        let t392 = t137 * t137;
        let t394 = 1.0 / t239 / t47 / t392;
        let t403 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 / t21 / t62 * t52 + 8.0 / 45.0 * t7 * t201 * t81 - t7 * t108 * t143 / 5.0 + 2.0 / 5.0 * t7 * t58 * t247 + 3.0 / 20.0 * t7 * t23 * (13090.0 / 6561.0 * t30 * t32 / t22 / t355 * t48 - 179585.0 / 629856.0 * t70 * t32 / t22 / t362 * t77 + 14245.0 / 5038848.0 * t129 * t32 / t22 / t130 / t355 * t139 - 485.0 / 0.60466176e8 * t234 * t32 / t22 / t130 / t362 * t243 + 5.0 / 0.725594112e9 * t29 * t232 * t183 * t32 / t22 / t385 / t355 * t394));
        let tv4rho40 = 2.0 * rho[ip] * t403 + 8.0 * t252;
        v4rho4[ip] += tv4rho40;
        let t445 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t201 * t99 - t7 * t108 * t170 / 10.0 + 3.0 / 10.0 * t7 * t58 * t282 + 3.0 / 20.0 * t7 * t23 * (-770.0 / 2187.0 * t29 * t215 + 17605.0 / 209952.0 * t156 * t220 * t76 * t41 - 6335.0 / 6718464.0 * t156 * t227 * t136 * t269 + 925.0 / 0.322486272e9 * t156 * t237 * t240 * t242 * t273 - 5.0 / 0.1934917632e10 * t29 * t232 * t69 * t32 / t22 / t385 / t211 * t394));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t445 + 6.0 * t287;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t480 = piecewise3(t2, 0.0, -t7 * t108 * t192 / 30.0 + t7 * t58 * t309 / 5.0 + 3.0 / 20.0 * t7 * t23 * (-175.0 / 8748.0 * t156 * t264 * t179 + 1295.0 / 4478976.0 * t156 * t268 * t297 - 95.0 / 0.95551488e8 * t156 * t277 * t240 * t242 * t232 + 5.0 / 0.5159780352e10 * t29 * t232 * t40 * t32 / t22 / t385 / t42 * t394));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t480 + 4.0 * t314;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t509 = piecewise3(t2, 0.0, t7 * t58 * t333 / 10.0 + 3.0 / 20.0 * t7 * t23 * (-355.0 / 4478976.0 * t156 * t296 * t318 + 95.0 / 0.286654464e9 * t156 * t304 * t240 * t242 * t128 + 5.0 / 1458.0 * t156 * t157 * t322 - 5.0 / 0.13759414272e11 * t234 * t32 / t22 / t385 / t62 * t394));
        let tv4rhosigma30 = 2.0 * rho[ip] * t509 + 2.0 * t337;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t535 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (-5.0 / 0.47775744e8 * t156 * t328 * t240 * t242 * t161 + 55.0 / 2985984.0 * t156 * t317 * t138 * t40 - 5.0 / 15552.0 * t156 * t178 * t38 + 5.0 / 0.36691771392e11 * t274 * t32 / t22 / t385 / t33 * t394));
        let tv4sigma40 = 2.0 * rho[ip] * t535;
        v4sigma4[ip] += tv4sigma40;
    }
}
