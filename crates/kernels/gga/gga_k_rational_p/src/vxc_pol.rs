//! GGA_K_RATIONAL_P vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_rational_p_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_C2: f64,
    param_p: f64,
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
        let t32 = 1.0 / param_p;
        let t34 = M_CBRT6;
        let t35 = param_C2 * t32 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3::<f64>(t36);
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t41 = rho0 * rho0;
        let t42 = pow_1_3::<f64>(rho0);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t49 = 1.0 + t35 * t39 * sigma0 * t45 / 24.0;
        let t50 = f64::powf(t49, -param_p);
        let t51 = t31 * t50;
        let t52 = t6 * t51;
        let t54 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t52);
        let t55 = rho1 <= dens_threshold;
        let t56 = -t17;
        let t58 = piecewise5::<f64>(t15, t12, t11, t16, t56 * t8);
        let t59 = 1.0 + t58;
        let t60 = t59 <= zeta_threshold;
        let t61 = pow_1_3::<f64>(t59);
        let t62 = t61 * t61;
        let t64 = piecewise3::<f64>(t60, t24, t62 * t59);
        let t65 = t64 * t30;
        let t67 = rho1 * rho1;
        let t68 = pow_1_3::<f64>(rho1);
        let t69 = t68 * t68;
        let t71 = 1.0 / t69 / t67;
        let t75 = 1.0 + t35 * t39 * sigma2 * t71 / 24.0;
        let t76 = f64::powf(t75, -param_p);
        let t77 = t65 * t76;
        let t78 = t6 * t77;
        let t80 = piecewise3::<f64>(t55, 0.0, 3.0 / 20.0 * t78);
        let tzk0 = t54 + t80;
        zk[ip] += tzk0;
        let t81 = t7 * t7;
        let t82 = 1.0 / t81;
        let t83 = t17 * t82;
        let t85 = piecewise5::<f64>(t11, 0.0, t15, 0.0, t8 - t83);
        let t88 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t85);
        let t89 = t88 * t30;
        let t90 = t89 * t50;
        let t91 = t6 * t90;
        let t93 = 1.0 / t29;
        let t94 = t28 * t93;
        let t95 = t94 * t50;
        let t96 = t6 * t95;
        let t97 = t96 / 10.0;
        let t98 = param_C2 * t34;
        let t99 = t98 * t39;
        let t100 = t41 * rho0;
        let t102 = 1.0 / t43 / t100;
        let t104 = 1.0 / t49;
        let t106 = t99 * sigma0 * t102 * t104;
        let t110 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t91 + t97 + t52 * t106 / 60.0);
        let t111 = t56 * t82;
        let t113 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -t8 - t111);
        let t116 = piecewise3::<f64>(t60, 0.0, 5.0 / 3.0 * t62 * t113);
        let t117 = t116 * t30;
        let t118 = t117 * t76;
        let t119 = t6 * t118;
        let t121 = t64 * t93;
        let t122 = t121 * t76;
        let t123 = t6 * t122;
        let t124 = t123 / 10.0;
        let t126 = piecewise3::<f64>(t55, 0.0, 3.0 / 20.0 * t119 + t124);
        let tvrho0 = t54 + t80 + t7 * (t110 + t126);
        vrho[ip * 2] += tvrho0;
        let t130 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -t8 - t83);
        let t133 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t130);
        let t134 = t133 * t30;
        let t135 = t134 * t50;
        let t136 = t6 * t135;
        let t139 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t136 + t97);
        let t141 = piecewise5::<f64>(t15, 0.0, t11, 0.0, t8 - t111);
        let t144 = piecewise3::<f64>(t60, 0.0, 5.0 / 3.0 * t62 * t141);
        let t145 = t144 * t30;
        let t146 = t145 * t76;
        let t147 = t6 * t146;
        let t149 = t67 * rho1;
        let t151 = 1.0 / t69 / t149;
        let t153 = 1.0 / t75;
        let t155 = t99 * sigma2 * t151 * t153;
        let t159 = piecewise3::<f64>(t55, 0.0, 3.0 / 20.0 * t147 + t124 + t78 * t155 / 60.0);
        let tvrho1 = t54 + t80 + t7 * (t139 + t159);
        vrho[ip * 2 + 1] += tvrho1;
        let t164 = t98 * t39 * t45 * t104;
        let t167 = piecewise3::<f64>(t1, 0.0, -t52 * t164 / 160.0);
        let tvsigma0 = t7 * t167;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t170 = t98 * t39 * t71 * t153;
        let t173 = piecewise3::<f64>(t55, 0.0, -t78 * t170 / 160.0);
        let tvsigma2 = t7 * t173;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
