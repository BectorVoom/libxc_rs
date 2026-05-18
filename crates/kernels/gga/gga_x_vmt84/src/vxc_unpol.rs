//! GGA_X_VMT84 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt84.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_vmt84_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
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
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t36 = param_alpha * t20 * t25;
        let t37 = sigma[ip] * t29;
        let t38 = t37 * t33;
        let t41 = f64::exp(-t36 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t34 * t47;
        let t51 = t20 * t20;
        let t54 = 1.0 / t23 / t22;
        let t55 = param_alpha * t51 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t28;
        let t58 = t30 * t30;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t18 / t59;
        let t65 = f64::exp(-t55 * t57 * t61 / 288.0);
        let t68 = (1.0 - t65) * t51 * t24;
        let t69 = 1.0 / sigma[ip];
        let t70 = t69 * t28;
        let t74 = t27 * t48 / 24.0 + 2.0 * t68 * t70 * t32 + t65;
        let t78 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t31;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t88 = t29 * t86 * t47;
        let t91 = param_mu * t51;
        let t92 = t54 * t56;
        let t93 = t91 * t92;
        let t94 = t58 * t30;
        let t96 = 1.0 / t18 / t94;
        let t97 = t28 * t96;
        let t98 = param_alpha * t41;
        let t99 = t98 * t46;
        let t103 = param_mu * param_mu;
        let t104 = t103 * t51;
        let t105 = t104 * t92;
        let t106 = t45 * t45;
        let t107 = 1.0 / t106;
        let t108 = t41 * t107;
        let t109 = t97 * t108;
        let t112 = t86 * t65;
        let t116 = t31 * rho[ip];
        let t120 = t96 * t65;
        let t124 = -t27 * t88 / 9.0 + t93 * t97 * t99 / 108.0 + t105 * t109 / 108.0 - 2.0 / 9.0 * t36 * t37 * t112 + 16.0 / 3.0 * t68 * t70 * t116 + t55 * t57 * t120 / 54.0;
        let t129 = piecewise3::<f64>(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t124);
        let tvrho0 = 2.0 * rho[ip] * t129 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t134 = t54 * sigma[ip];
        let t136 = t28 * t61;
        let t141 = t136 * t108;
        let t147 = 1.0 / t56;
        let t148 = t147 * t28;
        let t152 = sigma[ip] * t28;
        let t157 = t42 * t48 / 24.0 - t91 * t134 * t136 * t99 / 288.0 - t104 * t134 * t141 / 288.0 + t36 * t34 * t65 / 12.0 - 2.0 * t68 * t148 * t32 - t55 * t152 * t61 * t65 / 144.0;
        let t161 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
    }
}
