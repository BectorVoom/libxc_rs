//! GGA_X_NCAP vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ncap_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_mu: f64,
    param_zeta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t31 = t28 * t30;
        let t33 = t25 * t31 / 12.0;
        let t34 = f64::tanh(t33);
        let t35 = param_mu * t34;
        let t36 = f64::ln(t33 + f64::sqrt(t33 * t33 + 1.0));
        let t37 = 1.0 - param_zeta;
        let t39 = t37 * t21 * t24;
        let t40 = 1.0 + t33;
        let t41 = f64::ln(t40);
        let t42 = t30 * t41;
        let t46 = param_zeta * t21 * t24;
        let t51 = 1.0 + param_alpha * (t39 * t28 * t42 / 12.0 + t46 * t31 / 12.0);
        let t52 = t36 * t51;
        let t53 = param_beta * t34;
        let t55 = t53 * t36 + 1.0;
        let t56 = 1.0 / t55;
        let t57 = t52 * t56;
        let t59 = t35 * t57 + 1.0;
        let t63 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t59);
        let tzk0 = 2.0 * t63;
        zk[ip] += tzk0;
        let t64 = t18 * t18;
        let t66 = t17 / t64;
        let t70 = param_mu * t21;
        let t71 = t24 * t26;
        let t72 = t71 * t27;
        let t73 = t70 * t72;
        let t74 = rho[ip] * rho[ip];
        let t76 = 1.0 / t18 / t74;
        let t77 = t34 * t34;
        let t78 = 1.0 - t77;
        let t79 = t76 * t78;
        let t80 = t79 * t57;
        let t84 = t35 * t25 * t26;
        let t85 = t27 * t76;
        let t86 = t23 * t23;
        let t87 = 1.0 / t86;
        let t88 = t20 * t87;
        let t89 = t27 * t27;
        let t90 = sigma[ip] * t89;
        let t92 = 1.0 / t64 / t74;
        let t96 = 6.0 * t88 * t90 * t92 + 144.0;
        let t97 = f64::sqrt(t96);
        let t98 = 1.0 / t97;
        let t100 = t98 * t51 * t56;
        let t101 = t85 * t100;
        let t104 = t35 * t36;
        let t105 = t76 * t41;
        let t110 = t37 * t20 * t87;
        let t111 = t74 * rho[ip];
        let t113 = 1.0 / t64 / t111;
        let t114 = 1.0 / t40;
        let t115 = t113 * t114;
        let t119 = t28 * t76;
        let t122 = -t39 * t28 * t105 / 9.0 - t110 * t90 * t115 / 18.0 - t46 * t119 / 9.0;
        let t123 = param_alpha * t122;
        let t124 = t123 * t56;
        let t126 = t55 * t55;
        let t127 = 1.0 / t126;
        let t128 = t51 * t127;
        let t129 = param_beta * t21;
        let t130 = t129 * t71;
        let t131 = t78 * t36;
        let t132 = t85 * t131;
        let t135 = t53 * t25;
        let t136 = t76 * t98;
        let t140 = -t130 * t132 / 9.0 - 4.0 / 3.0 * t135 * t28 * t136;
        let t141 = t128 * t140;
        let t143 = -t73 * t80 / 9.0 - 4.0 / 3.0 * t84 * t101 + t104 * t124 - t104 * t141;
        let t148 = piecewise3(t2, 0.0, -t6 * t66 * t59 / 8.0 - 3.0 / 8.0 * t6 * t19 * t143);
        let tvrho0 = 2.0 * rho[ip] * t148 + 2.0 * t63;
        vrho[ip] += tvrho0;
        let t151 = 1.0 / t26;
        let t152 = t24 * t151;
        let t153 = t152 * t27;
        let t154 = t70 * t153;
        let t155 = t30 * t78;
        let t156 = t155 * t57;
        let t160 = t35 * t25 * t151;
        let t161 = t27 * t30;
        let t162 = t161 * t100;
        let t165 = t151 * t27;
        let t169 = t89 * t92;
        let t173 = t165 * t30;
        let t176 = t39 * t165 * t42 / 24.0 + t110 * t169 * t114 / 48.0 + t46 * t173 / 24.0;
        let t177 = param_alpha * t176;
        let t178 = t177 * t56;
        let t180 = t129 * t152;
        let t181 = t161 * t131;
        let t184 = t30 * t98;
        let t188 = t180 * t181 / 24.0 + t135 * t165 * t184 / 2.0;
        let t189 = t128 * t188;
        let t191 = t154 * t156 / 24.0 + t160 * t162 / 2.0 + t104 * t178 - t104 * t189;
        let t195 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t191);
        let tvsigma0 = 2.0 * rho[ip] * t195;
        vsigma[ip] += tvsigma0;
    }
}
