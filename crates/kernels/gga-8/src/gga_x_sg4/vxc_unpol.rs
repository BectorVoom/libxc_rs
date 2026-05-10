//! GGA_X_SG4 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 46 shared lines across all orders.
//! Delta: 24 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_sg4_vxc_unpol(
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
        // --- shared preamble (46 lines) ---
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
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 - 0.3123398257303946694e-2 * t34;
        let t37 = t20 * t20;
        let t38 = t21 * t21;
        let t39 = t38 * t21;
        let t41 = 1.0 / t22 / t39;
        let t42 = t37 * t41;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t43;
        let t45 = t44 * sigma[ip];
        let t47 = t29 * t29;
        let t48 = t47 * rho[ip];
        let t49 = t47 * t47;
        let t50 = t49 * t48;
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 - 0.14268491327672029207e-10 * t42 * t45 * t26 * t52;
        let t57 = 1.0 / t56;
        let t61 = 1.0 + 0.37270642201834862386e-1 * t34;
        let t64 = 0.1804e1 - 0.56028717948717948718e0 * t36 * t57 - 0.24371282051282051282e0 / t61;
        let t68 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t64);
        let tzk0 = 2.0 * t68;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (24 lines) ---
        let t70 = t17 / t30;
        let t74 = t25 * sigma[ip];
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t79 = t27 * t77 * t57;
        let t82 = t56 * t56;
        let t83 = 1.0 / t82;
        let t85 = t36 * t83 * t37;
        let t86 = t41 * t45;
        let t87 = t47 * t29;
        let t88 = t49 * t87;
        let t91 = t26 / t18 / t88;
        let t95 = t61 * t61;
        let t97 = 1.0 / t95 * t20;
        let t98 = t97 * t24;
        let t102 = -0.46666666666666666667e-2 * t74 * t79 + 0.10659270348691522892e-9 * t85 * t86 * t91 - 0.24222222222222222223e-1 * t98 * t28 * t77;
        let t107 = piecewise3(t2, 0.0, -t6 * t70 * t64 / 8.0 - 3.0 / 8.0 * t6 * t19 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t68;
        vrho[ip] += tvrho0;
        let t114 = t41 * t44;
        let t115 = t26 * t52;
        let t119 = t24 * t27;
        let t123 = 0.175e-2 * t25 * t27 * t32 * t57 - 0.39972263807593210847e-10 * t85 * t114 * t115 + 0.90833333333333333335e-2 * t97 * t119 * t32;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
    }
}
