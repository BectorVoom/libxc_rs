//! GGA_X_PBEINT vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 38 shared lines across all orders.
//! Delta: 42 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_pbeint_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (38 lines) ---
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
        let t20 = param_muPBE - param_muGE;
        let t21 = t20 * param_alpha;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = t21 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t18 * t18;
        let t35 = 1.0 / t33 / t32;
        let t38 = t31 * t35;
        let t41 = 1.0 + param_alpha * t22 * t26 * t38 / 24.0;
        let t42 = 1.0 / t41;
        let t43 = t35 * t42;
        let t48 = (param_muGE + t28 * t31 * t43 / 24.0) * t22;
        let t49 = t48 * t26;
        let t52 = param_kappa + t49 * t38 / 24.0;
        let t57 = 1.0 + param_kappa * (1.0 - param_kappa / t52);
        let t61 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t57);
        let tzk0 = 2.0 * t61;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (42 lines) ---
        let t62 = 1.0 / t33;
        let t63 = t17 * t62;
        let t67 = t6 * t17;
        let t68 = param_kappa * param_kappa;
        let t69 = t18 * t68;
        let t70 = t52 * t52;
        let t71 = 1.0 / t70;
        let t72 = t32 * rho[ip];
        let t74 = 1.0 / t33 / t72;
        let t75 = t74 * t42;
        let t79 = param_alpha * param_alpha;
        let t80 = t20 * t79;
        let t81 = t22 * t22;
        let t83 = 1.0 / t24 / t23;
        let t84 = t81 * t83;
        let t85 = t80 * t84;
        let t86 = sigma[ip] * sigma[ip];
        let t87 = t86 * t29;
        let t88 = t32 * t32;
        let t89 = t88 * t32;
        let t91 = 1.0 / t18 / t89;
        let t92 = t41 * t41;
        let t93 = 1.0 / t92;
        let t94 = t91 * t93;
        let t99 = (-t28 * t31 * t75 / 9.0 + t85 * t87 * t94 / 108.0) * t22;
        let t100 = t99 * t26;
        let t103 = t31 * t74;
        let t106 = t100 * t38 / 24.0 - t49 * t103 / 9.0;
        let t107 = t71 * t106;
        let t112 = piecewise3(t2, 0.0, -t6 * t63 * t57 / 8.0 - 3.0 / 8.0 * t67 * t69 * t107);
        let tvrho0 = 2.0 * rho[ip] * t112 + 2.0 * t61;
        vrho[ip] += tvrho0;
        let t115 = t21 * t22;
        let t116 = t26 * t30;
        let t121 = t88 * rho[ip];
        let t124 = 1.0 / t18 / t121 * t93;
        let t129 = (t115 * t116 * t43 / 24.0 - t85 * sigma[ip] * t29 * t124 / 288.0) * t22;
        let t130 = t129 * t26;
        let t132 = t116 * t35;
        let t135 = t130 * t38 / 24.0 + t48 * t132 / 24.0;
        let t136 = t71 * t135;
        let t140 = piecewise3(t2, 0.0, -3.0 / 8.0 * t67 * t69 * t136);
        let tvsigma0 = 2.0 * rho[ip] * t140;
        vsigma[ip] += tvsigma0;
    }
}
