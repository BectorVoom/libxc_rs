//! GGA_X_LSRPBE vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lsrpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lsrpbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = 1.0 / param_kappa;
        let t45 = f64::exp(-t34 * sigma0 * t39 * t41 / 24.0);
        let t48 = param_kappa + 1.0;
        let t49 = param_alpha * t28;
        let t50 = t33 * sigma0;
        let t54 = f64::exp(-t49 * t50 * t39 / 24.0);
        let t57 = 1.0 + param_kappa * (1.0 - t45) - t48 * (1.0 - t54);
        let t61 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t16;
        let t65 = piecewise5::<f64>(t14, t11, t10, t15, t63 * t7);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3::<f64>(t66);
        let t70 = piecewise3::<f64>(t67, t22, t68 * t66);
        let t71 = t70 * t26;
        let t72 = rho1 * rho1;
        let t73 = pow_1_3::<f64>(rho1);
        let t74 = t73 * t73;
        let t76 = 1.0 / t74 / t72;
        let t81 = f64::exp(-t34 * sigma2 * t76 * t41 / 24.0);
        let t84 = t33 * sigma2;
        let t88 = f64::exp(-t49 * t84 * t76 / 24.0);
        let t91 = 1.0 + param_kappa * (1.0 - t81) - t48 * (1.0 - t88);
        let t95 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t91);
        let tzk0 = t61 + t95;
        zk[ip] += tzk0;
        let t96 = t6 * t6;
        let t97 = 1.0 / t96;
        let t98 = t16 * t97;
        let t100 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t98);
        let t103 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t100);
        let t104 = t103 * t26;
        let t108 = t26 * t26;
        let t109 = 1.0 / t108;
        let t110 = t25 * t109;
        let t113 = t5 * t110 * t57 / 8.0;
        let t114 = t35 * rho0;
        let t116 = 1.0 / t37 / t114;
        let t121 = t48 * param_alpha * t28;
        let t126 = t121 * t50 * t116 * t54 / 9.0 - t34 * sigma0 * t116 * t45 / 9.0;
        let t131 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t104 * t57 - t113 - 3.0 / 8.0 * t5 * t27 * t126);
        let t132 = t63 * t97;
        let t134 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t132);
        let t137 = piecewise3::<f64>(t67, 0.0, 4.0 / 3.0 * t68 * t134);
        let t138 = t137 * t26;
        let t142 = t70 * t109;
        let t145 = t5 * t142 * t91 / 8.0;
        let t147 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t138 * t91 - t145);
        let tvrho0 = t61 + t95 + t6 * (t131 + t147);
        vrho[ip * 2] += tvrho0;
        let t151 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t98);
        let t154 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t151);
        let t155 = t154 * t26;
        let t160 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t155 * t57 - t113);
        let t162 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t132);
        let t165 = piecewise3::<f64>(t67, 0.0, 4.0 / 3.0 * t68 * t162);
        let t166 = t165 * t26;
        let t170 = t72 * rho1;
        let t172 = 1.0 / t74 / t170;
        let t180 = t121 * t84 * t172 * t88 / 9.0 - t34 * sigma2 * t172 * t81 / 9.0;
        let t185 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t166 * t91 - t145 - 3.0 / 8.0 * t5 * t71 * t180);
        let tvrho1 = t61 + t95 + t6 * (t160 + t185);
        vrho[ip * 2 + 1] += tvrho1;
        let t188 = t33 * t39;
        let t194 = -t121 * t188 * t54 / 24.0 + t29 * t188 * t45 / 24.0;
        let t198 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t194);
        let tvsigma0 = t6 * t198;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t199 = t33 * t76;
        let t205 = -t121 * t199 * t88 / 24.0 + t29 * t199 * t81 / 24.0;
        let t209 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t205);
        let tvsigma2 = t6 * t209;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
