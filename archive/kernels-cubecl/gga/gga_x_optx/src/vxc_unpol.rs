//! GGA_X_OPTX vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_optx.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_optx_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_gamma: f64,
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
        let t20 = param_gamma * param_gamma;
        let t21 = param_b * t20;
        let t22 = sigma[ip] * sigma[ip];
        let t23 = t21 * t22;
        let t24 = M_CBRT2;
        let t25 = rho[ip] * rho[ip];
        let t26 = t25 * t25;
        let t27 = t26 * rho[ip];
        let t32 = t24 * t24;
        let t33 = t18 * t18;
        let t35 = 1.0 / t33 / t25;
        let t38 = param_gamma * sigma[ip] * t32 * t35 + 1.0;
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t24 / t18 / t27 * t40;
        let t44 = 2.0 * t23 * t41 + param_a;
        let t48 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t50 = t17 / t33;
        let t54 = t26 * t25;
        let t58 = t24 / t18 / t54 * t40;
        let t62 = param_b * t20 * param_gamma;
        let t63 = t22 * sigma[ip];
        let t64 = t26 * t26;
        let t65 = t64 * rho[ip];
        let t66 = 1.0 / t65;
        let t69 = 1.0 / t39 / t38;
        let t73 = -32.0 / 3.0 * t23 * t58 + 64.0 / 3.0 * t62 * t63 * t66 * t69;
        let t78 = piecewise3::<f64>(t2, 0.0, -t6 * t50 * t44 / 8.0 - 3.0 / 8.0 * t6 * t19 * t73);
        let tvrho0 = 2.0 * rho[ip] * t78 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t81 = t21 * sigma[ip];
        let t84 = 1.0 / t64;
        let t89 = -8.0 * t62 * t22 * t84 * t69 + 4.0 * t81 * t41;
        let t93 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
    }
}
