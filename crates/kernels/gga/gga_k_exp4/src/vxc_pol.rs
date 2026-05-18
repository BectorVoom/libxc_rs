//! GGA_K_EXP4 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_exp4_vxc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3::<f64>(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3::<f64>(t21, t24, t26 * t20);
        let t29 = pow_1_3::<f64>(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3::<f64>(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3::<f64>(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = f64::exp(-0.83254166666666666664e1 * t37 * sigma0 * t42);
        let t48 = t32 * t32;
        let t51 = t48 / t34 / t33;
        let t52 = sigma0 * sigma0;
        let t53 = t38 * t38;
        let t54 = t53 * rho0;
        let t56 = 1.0 / t39 / t54;
        let t60 = f64::exp(-0.75479166666666666666e-2 * t51 * t52 * t56);
        let t62 = 0.20788e1 - 0.8524e0 * t46 - 0.12264e1 * t60;
        let t66 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t62);
        let t67 = rho1 <= dens_threshold;
        let t68 = -t17;
        let t70 = piecewise5::<f64>(t15, t12, t11, t16, t68 * t8);
        let t71 = 1.0 + t70;
        let t72 = t71 <= zeta_threshold;
        let t73 = pow_1_3::<f64>(t71);
        let t74 = t73 * t73;
        let t76 = piecewise3::<f64>(t72, t24, t74 * t71);
        let t77 = t76 * t30;
        let t78 = rho1 * rho1;
        let t79 = pow_1_3::<f64>(rho1);
        let t80 = t79 * t79;
        let t82 = 1.0 / t80 / t78;
        let t86 = f64::exp(-0.83254166666666666664e1 * t37 * sigma2 * t82);
        let t88 = sigma2 * sigma2;
        let t89 = t78 * t78;
        let t90 = t89 * rho1;
        let t92 = 1.0 / t79 / t90;
        let t96 = f64::exp(-0.75479166666666666666e-2 * t51 * t88 * t92);
        let t98 = 0.20788e1 - 0.8524e0 * t86 - 0.12264e1 * t96;
        let t102 = piecewise3::<f64>(t67, 0.0, 3.0 / 20.0 * t6 * t77 * t98);
        let tzk0 = t66 + t102;
        zk[ip] += tzk0;
        let t103 = t7 * t7;
        let t104 = 1.0 / t103;
        let t105 = t17 * t104;
        let t107 = piecewise5::<f64>(t11, 0.0, t15, 0.0, t8 - t105);
        let t110 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t107);
        let t111 = t110 * t30;
        let t115 = 1.0 / t29;
        let t116 = t28 * t115;
        let t119 = t6 * t116 * t62 / 10.0;
        let t120 = t38 * rho0;
        let t122 = 1.0 / t40 / t120;
        let t127 = t53 * t38;
        let t129 = 1.0 / t39 / t127;
        let t134 = -0.1892422711111111111e2 * t37 * sigma0 * t122 * t46 - 0.49369413333333333333e-1 * t51 * t52 * t129 * t60;
        let t139 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t111 * t62 + t119 + 3.0 / 20.0 * t6 * t31 * t134);
        let t140 = t68 * t104;
        let t142 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -t8 - t140);
        let t145 = piecewise3::<f64>(t72, 0.0, 5.0 / 3.0 * t74 * t142);
        let t146 = t145 * t30;
        let t150 = t76 * t115;
        let t153 = t6 * t150 * t98 / 10.0;
        let t155 = piecewise3::<f64>(t67, 0.0, 3.0 / 20.0 * t6 * t146 * t98 + t153);
        let tvrho0 = t66 + t102 + t7 * (t139 + t155);
        vrho[ip * 2] += tvrho0;
        let t159 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -t8 - t105);
        let t162 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t159);
        let t163 = t162 * t30;
        let t168 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t163 * t62 + t119);
        let t170 = piecewise5::<f64>(t15, 0.0, t11, 0.0, t8 - t140);
        let t173 = piecewise3::<f64>(t72, 0.0, 5.0 / 3.0 * t74 * t170);
        let t174 = t173 * t30;
        let t178 = t78 * rho1;
        let t180 = 1.0 / t80 / t178;
        let t185 = t89 * t78;
        let t187 = 1.0 / t79 / t185;
        let t192 = -0.1892422711111111111e2 * t37 * sigma2 * t180 * t86 - 0.49369413333333333333e-1 * t51 * t88 * t187 * t96;
        let t197 = piecewise3::<f64>(t67, 0.0, 3.0 / 20.0 * t6 * t174 * t98 + t153 + 3.0 / 20.0 * t6 * t77 * t192);
        let tvrho1 = t66 + t102 + t7 * (t168 + t197);
        vrho[ip * 2 + 1] += tvrho1;
        let t207 = 0.70965851666666666664e1 * t37 * t42 * t46 + 0.1851353e-1 * t51 * sigma0 * t56 * t60;
        let t211 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t207);
        let tvsigma0 = t7 * t211;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t219 = 0.70965851666666666664e1 * t37 * t82 * t86 + 0.1851353e-1 * t51 * sigma2 * t92 * t96;
        let t223 = piecewise3::<f64>(t67, 0.0, 3.0 / 20.0 * t6 * t77 * t219);
        let tvsigma2 = t7 * t223;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
