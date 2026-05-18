//! GGA_X_AIRY vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_airy_vxc_unpol(
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
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t32 = t25 * t28 * t30;
        let t33 = f64::powf(t32, 0.2626712e1);
        let t35 = 1.0 + 0.13471619689594796103e-3 * t33;
        let t36 = f64::powf(t35, -0.657946e0);
        let t39 = f64::powf(t32, 0.3217063e1);
        let t41 = f64::powf(t32, 0.3223476e1);
        let t43 = 1.0 - 0.45212413010769857073e-1 * t39 + 0.45402221956620378581e-1 * t41;
        let t44 = f64::powf(t32, 0.3473804e1);
        let t46 = 1.0 + 0.47702180224903349918e-3 * t44;
        let t47 = 1.0 / t46;
        let t49 = 0.60146019220211109872e-4 * t33 * t36 + t43 * t47;
        let t53 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t54 = t18 * t18;
        let t56 = t17 / t54;
        let t60 = f64::powf(t32, 0.1626712e1);
        let t62 = t60 * t36 * t21;
        let t63 = t24 * t26;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t18 / t64;
        let t67 = t27 * t66;
        let t68 = t63 * t67;
        let t71 = f64::powf(t32, 0.4253424e1);
        let t72 = f64::powf(t35, -0.1657946e1);
        let t74 = t71 * t72 * t21;
        let t77 = f64::powf(t32, 0.2217063e1);
        let t79 = t77 * t21 * t24;
        let t80 = t28 * t66;
        let t83 = f64::powf(t32, 0.2223476e1);
        let t85 = t83 * t21 * t24;
        let t88 = 0.19393490805022174494e0 * t79 * t80 - 0.19513729709845177529e0 * t85 * t80;
        let t90 = t46 * t46;
        let t91 = 1.0 / t90;
        let t92 = t43 * t91;
        let t93 = f64::powf(t32, 0.2473804e1);
        let t94 = t93 * t21;
        let t95 = t92 * t94;
        let t98 = -0.21064836058394555311e-3 * t62 * t68 + 0.18671024483029835192e-7 * t74 * t68 + t88 * t47 + 0.22094403263198687541e-2 * t95 * t68;
        let t103 = piecewise3::<f64>(t2, 0.0, -t6 * t56 * t49 / 8.0 - 3.0 / 8.0 * t6 * t19 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t106 = 1.0 / t26;
        let t107 = t24 * t106;
        let t108 = t27 * t30;
        let t109 = t107 * t108;
        let t114 = t106 * t27;
        let t115 = t114 * t30;
        let t120 = -0.72725590518833154352e-1 * t79 * t115 + 0.73176486411919415733e-1 * t85 * t115;
        let t124 = 0.78993135218979582417e-4 * t62 * t109 - 0.7001634181136188197e-8 * t74 * t109 + t120 * t47 - 0.82854012236995078279e-3 * t95 * t109;
        let t128 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
    }
}
