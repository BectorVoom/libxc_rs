//! GGA_C_WL fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wl_fxc_unpol(
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
        let t1 = f64::sqrt(sigma[ip]);
        let t2 = pow_1_3::<f64>(rho[ip]);
        let t4 = 1.0 / t2 / rho[ip];
        let t5 = t1 * t4;
        let t7 = -0.7486e0 + 0.6001e-1 * t5;
        let t8 = M_CBRT2;
        let t9 = t1 * t8;
        let t12 = M_CBRT3;
        let t14 = pow_1_3::<f64>(1.0 / M_PI);
        let t15 = t12 * t14;
        let t16 = M_CBRT4;
        let t17 = t16 * t16;
        let t18 = 1.0 / t2;
        let t22 = 0.360073e1 + 0.18e1 * t9 * t4 + t15 * t17 * t18 / 4.0;
        let t23 = 1.0 / t22;
        let tzk0 = t7 * t23;
        zk[ip] += tzk0;
        let t26 = rho[ip] * t7;
        let t27 = t22 * t22;
        let t28 = 1.0 / t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t2 / t29;
        let t37 = -0.24e1 * t9 * t31 - t15 * t17 * t4 / 12.0;
        let t38 = t28 * t37;
        let tvrho0 = tzk0 - 0.80013333333333333333e-1 * t5 * t23 - t26 * t38;
        vrho[ip] += tvrho0;
        let t40 = 1.0 / t1;
        let t41 = t18 * t40;
        let t44 = t18 * t7;
        let t46 = t28 * t40 * t8;
        let tvsigma0 = 0.30005e-1 * t41 * t23 - 0.9e0 * t44 * t46;
        vsigma[ip] += tvsigma0;
        let t49 = t1 * t31;
        let t52 = t7 * t28;
        let t58 = 1.0 / t27 / t22;
        let t59 = t37 * t37;
        let t60 = t58 * t59;
        let t63 = t29 * rho[ip];
        let t65 = 1.0 / t2 / t63;
        let t71 = 0.56e1 * t9 * t65 + t15 * t17 * t31 / 9.0;
        let t72 = t28 * t71;
        let tv2rho20 = 0.26671111111111111107e-1 * t49 * t23 - 2.0 * t52 * t37 + 0.16002666666666666667e0 * t5 * t38 + 2.0 * t26 * t60 - t26 * t72;
        v2rho2[ip] += tv2rho20;
        let t74 = t4 * t40;
        let t79 = t4 * t7;
        let t82 = t2 * t2;
        let t84 = 1.0 / t82 / t29;
        let t88 = t44 * t58;
        let t89 = t40 * t8;
        let t90 = t89 * t37;
        let tv2rhosigma0 = -0.10001666666666666667e-1 * t74 * t23 - 0.30005e-1 * t41 * t38 + 0.3e0 * t79 * t46 + 0.72012e-1 * t84 * t28 * t8 + 0.18e1 * t88 * t90;
        v2rhosigma[ip] += tv2rhosigma0;
        let t94 = 1.0 / t1 / sigma[ip];
        let t95 = t18 * t94;
        let t99 = 1.0 / t82 / rho[ip];
        let t100 = 1.0 / sigma[ip];
        let t101 = t99 * t100;
        let t102 = t28 * t8;
        let t105 = t99 * t7;
        let t107 = t8 * t8;
        let t108 = t58 * t100 * t107;
        let t112 = t28 * t94 * t8;
        let tv2sigma20 = -0.150025e-1 * t95 * t23 - 0.54009e-1 * t101 * t102 + 0.162e1 * t105 * t108 + 0.45e0 * t44 * t112;
        v2sigma2[ip] += tv2sigma20;
    }
}
