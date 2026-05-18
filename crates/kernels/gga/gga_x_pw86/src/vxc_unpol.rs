//! GGA_X_PW86 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw86_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        let t21 = param_aa * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = t20 * t20;
        let t38 = param_bb * t37;
        let t40 = 1.0 / t23 / t22;
        let t41 = t38 * t40;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t27;
        let t44 = t30 * t30;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t18 / t45;
        let t51 = t22 * t22;
        let t53 = param_cc / t51;
        let t54 = t42 * sigma[ip];
        let t55 = t44 * t44;
        let t56 = 1.0 / t55;
        let t60 = 1.0 + t26 * t29 * t33 / 24.0 + t41 * t43 * t47 / 288.0 + t53 * t54 * t56 / 576.0;
        let t61 = f64::powf(t60, 1.0 / 15.0);
        let t65 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t61);
        let tzk0 = 2.0 * t65;
        zk[ip] += tzk0;
        let t66 = 1.0 / t31;
        let t71 = t6 * t17;
        let t72 = t61 * t61;
        let t73 = t72 * t72;
        let t75 = t73 * t73;
        let t76 = t75 * t73 * t72;
        let t77 = 1.0 / t76;
        let t78 = t18 * t77;
        let t79 = t30 * rho[ip];
        let t81 = 1.0 / t31 / t79;
        let t85 = t44 * t30;
        let t87 = 1.0 / t18 / t85;
        let t91 = t55 * rho[ip];
        let t92 = 1.0 / t91;
        let t96 = -t26 * t29 * t81 / 9.0 - t41 * t43 * t87 / 54.0 - t53 * t54 * t92 / 72.0;
        let t101 = piecewise3::<f64>(t2, 0.0, -t6 * t17 * t66 * t61 / 8.0 - t71 * t78 * t96 / 40.0);
        let tvrho0 = 2.0 * rho[ip] * t101 + 2.0 * t65;
        vrho[ip] += tvrho0;
        let t104 = t25 * t28;
        let t108 = sigma[ip] * t27;
        let t115 = t21 * t104 * t33 / 24.0 + t41 * t108 * t47 / 144.0 + t53 * t42 * t56 / 192.0;
        let t119 = piecewise3::<f64>(t2, 0.0, -t71 * t78 * t115 / 40.0);
        let tvsigma0 = 2.0 * rho[ip] * t119;
        vsigma[ip] += tvsigma0;
    }
}
