//! GGA_X_LV_RPW86 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 48 shared lines across all orders.
//! Delta: 66 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lv_rpw86_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (48 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 + 0.39310185185185185185e-2 * t34;
        let t37 = sigma[ip] * sigma[ip];
        let t38 = t37 * sigma[ip];
        let t39 = t29 * t29;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t42 = t38 * t41;
        let t43 = 0.38818245400525142432e-6 * t42;
        let t44 = 1.0 + t43;
        let t45 = 1.0 / t44;
        let t48 = t20 * t20;
        let t51 = t48 / t22 / t21;
        let t52 = t37 * t26;
        let t53 = t39 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t60 = 1.0 + 0.77125000000000000002e-1 * t34 + 0.60173611111111111112e-1 * t51 * t52 * t55 + 0.29051303949887962426e-5 * t42;
        let t61 = f64::powf(t60, 1.0 / 15.0);
        let t62 = 0.115e1 + t43;
        let t63 = 1.0 / t62;
        let t64 = t61 * t63;
        let t67 = t36 * t45 + 0.38818245400525142432e-6 * t42 * t64;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        // --- vxc delta (44 lines) ---
        let t73 = t17 / t30;
        let t77 = t25 * sigma[ip];
        let t78 = t29 * rho[ip];
        let t80 = 1.0 / t30 / t78;
        let t81 = t27 * t80;
        let t82 = t81 * t45;
        let t85 = t44 * t44;
        let t86 = 1.0 / t85;
        let t87 = t36 * t86;
        let t88 = t40 * rho[ip];
        let t89 = 1.0 / t88;
        let t90 = t38 * t89;
        let t95 = t61 * t61;
        let t96 = t95 * t95;
        let t98 = t96 * t96;
        let t99 = t98 * t96 * t95;
        let t100 = 1.0 / t99;
        let t101 = t100 * t63;
        let t105 = t39 * t29;
        let t107 = 1.0 / t18 / t105;
        let t112 = -0.20566666666666666667e0 * t25 * t28 * t80 - 0.32092592592592592593e0 * t51 * t52 * t107 - 0.23241043159910369941e-4 * t90;
        let t113 = t101 * t112;
        let t116 = t37 * t37;
        let t117 = t116 * t37;
        let t118 = t40 * t40;
        let t120 = 1.0 / t118 / rho[ip];
        let t121 = t117 * t120;
        let t122 = t62 * t62;
        let t123 = 1.0 / t122;
        let t124 = t61 * t123;
        let t127 = -0.10482716049382716049e-1 * t77 * t82 + 0.31054596320420113946e-5 * t87 * t90 - 0.31054596320420113946e-5 * t90 * t64 + 0.25878830267016761621e-7 * t42 * t113 + 0.12054849407803131005e-11 * t121 * t124;
        let t132 = piecewise3(t2, 0.0, -t6 * t73 * t67 / 8.0 - 3.0 / 8.0 * t6 * t19 * t127);
        let tvrho0 = 2.0 * rho[ip] * t132 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t135 = t27 * t32;
        let t139 = t37 * t41;
        let t146 = sigma[ip] * t26;
        let t151 = 0.77125000000000000002e-1 * t25 * t135 + 0.12034722222222222222e0 * t51 * t146 * t55 + 0.87153911849663887278e-5 * t139;
        let t152 = t101 * t151;
        let t155 = t116 * sigma[ip];
        let t156 = 1.0 / t118;
        let t157 = t155 * t156;
        let t160 = 0.39310185185185185185e-2 * t25 * t135 * t45 - 0.1164547362015754273e-5 * t87 * t139 + 0.1164547362015754273e-5 * t139 * t64 + 0.25878830267016761621e-7 * t42 * t152 - 0.45205685279261741269e-12 * t157 * t124;
        let t164 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t160);
        let tvsigma0 = 2.0 * rho[ip] * t164;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (66 lines) ---
        let t169 = t17 / t30 / rho[ip];
        let t177 = 1.0 / t30 / t39;
        let t178 = t27 * t177;
        let t179 = t178 * t45;
        let t182 = t25 * t116;
        let t183 = t40 * t39;
        let t185 = 1.0 / t30 / t183;
        let t187 = t27 * t185 * t86;
        let t191 = 1.0 / t85 / t44;
        let t192 = t36 * t191;
        let t193 = t118 * t29;
        let t194 = 1.0 / t193;
        let t195 = t117 * t194;
        let t198 = t40 * t29;
        let t199 = 1.0 / t198;
        let t200 = t38 * t199;
        let t210 = 1.0 / t99 / t60;
        let t211 = t210 * t63;
        let t212 = t112 * t112;
        let t213 = t211 * t212;
        let t216 = t100 * t123;
        let t217 = t216 * t112;
        let t225 = 1.0 / t18 / t39 / t78;
        let t230 = 0.75411111111111111112e0 * t25 * t28 * t177 + 0.20325308641975308642e1 * t51 * t52 * t225 + 0.20916938843919332947e-3 * t200;
        let t231 = t101 * t230;
        let t234 = t116 * t116;
        let t235 = t234 * sigma[ip];
        let t236 = t118 * t198;
        let t237 = 1.0 / t236;
        let t238 = t235 * t237;
        let t240 = 1.0 / t122 / t62;
        let t241 = t61 * t240;
        let t244 = 0.3843662551440329218e-1 * t77 * t179 - 0.65107303051033873458e-7 * t182 * t187 + 0.19287759052485009608e-10 * t192 * t195 - 0.27949136688378102551e-4 * t87 * t200 + 0.27949136688378102551e-4 * t200 * t64 - 0.41406128427226818594e-6 * t90 * t113 - 0.30137123519507827512e-10 * t195 * t124 - 0.24153574915882310846e-7 * t42 * t213 + 0.16073132543737508006e-12 * t121 * t217 + 0.25878830267016761621e-7 * t42 * t231 + 0.74871696412556340349e-17 * t238 * t241;
        let t249 = piecewise3(t2, 0.0, t6 * t169 * t67 / 12.0 - t6 * t73 * t127 / 4.0 - 3.0 / 8.0 * t6 * t19 * t244);
        let tv2rho20 = 2.0 * rho[ip] * t249 + 4.0 * t132;
        v2rho2[ip] += tv2rho20;
        let t257 = t25 * t27;
        let t258 = t40 * t78;
        let t260 = 1.0 / t30 / t258;
        let t261 = t260 * t86;
        let t265 = t155 * t120;
        let t268 = t37 * t89;
        let t279 = t42 * t210;
        let t280 = t63 * t151;
        let t281 = t280 * t112;
        let t284 = t216 * t151;
        let t293 = -0.20566666666666666667e0 * t25 * t81 - 0.64185185185185185184e0 * t51 * t146 * t107 - 0.69723129479731109822e-4 * t268;
        let t294 = t101 * t293;
        let t300 = 1.0 / t118 / t88;
        let t301 = t234 * t300;
        let t304 = -0.10482716049382716049e-1 * t25 * t82 + 0.24415238644137702548e-7 * t257 * t261 * t38 - 0.72329096446818786032e-11 * t192 * t265 + 0.9316378896126034184e-5 * t87 * t268 - 0.9316378896126034184e-5 * t268 * t64 + 0.77636490801050284867e-7 * t139 * t113 + 0.10849364467022817905e-10 * t265 * t124 - 0.20703064213613409297e-6 * t90 * t152 - 0.24153574915882310846e-7 * t279 * t281 + 0.80365662718687540032e-13 * t121 * t284 + 0.25878830267016761621e-7 * t42 * t294 - 0.30137123519507827513e-13 * t157 * t217 - 0.28076886154708627631e-17 * t301 * t241;
        let t309 = piecewise3(t2, 0.0, -t6 * t73 * t160 / 8.0 - 3.0 / 8.0 * t6 * t19 * t304);
        let tv2rhosigma0 = 2.0 * rho[ip] * t309 + 2.0 * t164;
        v2rhosigma[ip] += tv2rhosigma0;
        let t313 = 1.0 / t30 / t198;
        let t314 = t313 * t86;
        let t318 = t116 * t156;
        let t321 = sigma[ip] * t41;
        let t330 = t151 * t151;
        let t331 = t211 * t330;
        let t340 = 0.12034722222222222222e0 * t51 * t26 * t55 + 0.17430782369932777456e-4 * t321;
        let t341 = t101 * t340;
        let t344 = t116 * t38;
        let t346 = 1.0 / t118 / t40;
        let t347 = t344 * t346;
        let t350 = -0.91557144915516384556e-8 * t257 * t314 * t37 + 0.27123411167557044763e-11 * t192 * t318 - 0.2329094724031508546e-5 * t87 * t321 + 0.2329094724031508546e-5 * t321 * t64 + 0.15527298160210056973e-6 * t139 * t152 - 0.36164548223409393015e-11 * t318 * t124 - 0.24153574915882310846e-7 * t42 * t331 - 0.60274247039015655025e-13 * t157 * t284 + 0.25878830267016761621e-7 * t42 * t341 + 0.10528832308015735362e-17 * t347 * t241;
        let t354 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t350);
        let tv2sigma20 = 2.0 * rho[ip] * t354;
        v2sigma2[ip] += tv2sigma20;
    }
}
