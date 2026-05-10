//! GGA_X_SOGGA11 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 52 shared lines across all orders.
//! Delta: 49 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_sogga11_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (52 lines) ---
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
        let t21 = param_a_1;
        let t22 = M_CBRT6;
        let t23 = param_mu * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = t25 * t25;
        let t27 = 1.0 / t26;
        let t28 = t23 * t27;
        let t29 = 1.0 / param_kappa;
        let t30 = t29 * sigma[ip];
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = rho[ip] * rho[ip];
        let t34 = t18 * t18;
        let t36 = 1.0 / t34 / t33;
        let t37 = t32 * t36;
        let t40 = t28 * t30 * t37 / 24.0;
        let t41 = 1.0 + t40;
        let t43 = 1.0 - 1.0 / t41;
        let t45 = param_a_2;
        let t46 = t43 * t43;
        let t48 = param_a_3;
        let t49 = t46 * t43;
        let t51 = param_a_4;
        let t52 = t46 * t46;
        let t54 = param_a_5;
        let t58 = param_b_1;
        let t59 = f64::exp(-t40);
        let t60 = 1.0 - t59;
        let t62 = param_b_2;
        let t63 = t60 * t60;
        let t65 = param_b_3;
        let t66 = t63 * t60;
        let t68 = param_b_4;
        let t69 = t63 * t63;
        let t71 = param_b_5;
        let t74 = t54 * t52 * t43 + t71 * t69 * t60 + t21 * t43 + t45 * t46 + t48 * t49 + t51 * t52 + t58 * t60 + t62 * t63 + t65 * t66 + t68 * t69 + param_a_0 + param_b_0;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (49 lines) ---
        let t80 = t17 / t34;
        let t84 = t41 * t41;
        let t85 = 1.0 / t84;
        let t87 = t21 * t85 * t23;
        let t88 = t27 * t29;
        let t89 = sigma[ip] * t32;
        let t90 = t33 * rho[ip];
        let t92 = 1.0 / t34 / t90;
        let t94 = t88 * t89 * t92;
        let t97 = t45 * t43;
        let t98 = t85 * param_mu;
        let t99 = t98 * t22;
        let t100 = t97 * t99;
        let t103 = t48 * t46;
        let t104 = t103 * t99;
        let t107 = t51 * t49;
        let t108 = t107 * t99;
        let t111 = t54 * t52;
        let t112 = t111 * t99;
        let t116 = t22 * t27;
        let t117 = t58 * param_mu * t116;
        let t118 = t32 * t92;
        let t119 = t118 * t59;
        let t120 = t30 * t119;
        let t123 = t62 * t60;
        let t124 = t123 * t28;
        let t127 = t65 * t63;
        let t128 = t127 * t28;
        let t131 = t68 * t66;
        let t132 = t131 * t28;
        let t135 = t71 * t69;
        let t136 = t135 * t28;
        let t139 = -t87 * t94 / 9.0 - 2.0 / 9.0 * t100 * t94 - t104 * t94 / 3.0 - 4.0 / 9.0 * t108 * t94 - 5.0 / 9.0 * t112 * t94 - t117 * t120 / 9.0 - 2.0 / 9.0 * t124 * t120 - t128 * t120 / 3.0 - 4.0 / 9.0 * t132 * t120 - 5.0 / 9.0 * t136 * t120;
        let t144 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t139);
        let tvrho0 = 2.0 * rho[ip] * t144 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t150 = t97 * t98;
        let t151 = t29 * t32;
        let t153 = t116 * t151 * t36;
        let t156 = t103 * t98;
        let t159 = t107 * t98;
        let t162 = t111 * t98;
        let t169 = t123 * t23;
        let t171 = t88 * t37 * t59;
        let t174 = t127 * t23;
        let t177 = t131 * t23;
        let t180 = t135 * t23;
        let t183 = t87 * t88 * t37 / 24.0 + t150 * t153 / 12.0 + t156 * t153 / 8.0 + t159 * t153 / 6.0 + 5.0 / 24.0 * t162 * t153 + t117 * t151 * t36 * t59 / 24.0 + t169 * t171 / 12.0 + t174 * t171 / 8.0 + t177 * t171 / 6.0 + 5.0 / 24.0 * t180 * t171;
        let t187 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t183);
        let tvsigma0 = 2.0 * rho[ip] * t187;
        vsigma[ip] += tvsigma0;
    }
}
