//! GGA_X_WC vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 26 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_wc_vxc_unpol(
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
        // --- shared preamble (41 lines) ---
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = t25 * sigma[ip];
        let t37 = t27 * t32;
        let t39 = f64::exp(-t34 / 24.0);
        let t40 = t37 * t39;
        let t43 = t20 * t20;
        let t46 = t43 / t22 / t21;
        let t47 = sigma[ip] * sigma[ip];
        let t49 = t29 * t29;
        let t50 = t49 * rho[ip];
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 + 0.27560657413756315278e-4 * t46 * t47 * t26 * t52;
        let t57 = f64::ln(t56);
        let t58 = 0.804e0 + 5.0 / 972.0 * t34 + 0.4002424276710846245e-2 * t36 * t40 + t57;
        let t61 = 0.1804e1 - 0.646416e0 / t58;
        let t65 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t61);
        let tzk0 = 2.0 * t65;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (26 lines) ---
        let t66 = 1.0 / t30;
        let t71 = t3 * t17;
        let t72 = t58 * t58;
        let t73 = 1.0 / t72;
        let t74 = t18 * t73;
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t81 = t27 * t77;
        let t82 = t81 * t39;
        let t85 = t46 * t47;
        let t86 = t49 * t29;
        let t88 = 1.0 / t18 / t86;
        let t89 = t26 * t88;
        let t90 = t89 * t39;
        let t93 = 1.0 / t56;
        let t94 = t89 * t93;
        let t97 = -10.0 / 729.0 * t25 * t28 * t77 - 0.10673131404562256653e-1 * t36 * t82 + 0.88942761704685472111e-3 * t85 * t90 - 0.14699017287336701482e-3 * t85 * t94;
        let t102 = piecewise3(t2, 0.0, -t6 * t17 * t66 * t61 / 8.0 - 0.16551095363746320496e0 * t71 * t74 * t97);
        let tvrho0 = 2.0 * rho[ip] * t102 + 2.0 * t65;
        vrho[ip] += tvrho0;
        let t109 = t46 * sigma[ip];
        let t110 = t26 * t52;
        let t111 = t110 * t39;
        let t114 = t110 * t93;
        let t117 = 5.0 / 972.0 * t25 * t37 + 0.4002424276710846245e-2 * t25 * t40 - 0.33353535639257052042e-3 * t109 * t111 + 0.55121314827512630556e-4 * t109 * t114;
        let t121 = piecewise3(t2, 0.0, -0.16551095363746320496e0 * t71 * t74 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
    }
}
