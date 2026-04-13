//! MGGA_K_PGSLB vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_pgslb_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_pgslb_beta: f64,
    param_pgslb_mu: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t47 = param_pgslb_mu * t33;
        let t48 = t37 * sigma0;
        let t52 = f64::exp(-t47 * t48 * t43 / 24.0);
        let t53 = t33 * t33;
        let t54 = param_pgslb_beta * t53;
        let t56 = 1.0 / t35 / t34;
        let t57 = lapl0 * lapl0;
        let t58 = t56 * t57;
        let t59 = t39 * rho0;
        let t61 = 1.0 / t40 / t59;
        let t65 = 5.0 / 72.0 * t38 * sigma0 * t43 + t52 + t54 * t58 * t61 / 576.0;
        let t69 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t18;
        let t73 = piecewise5(t16, t13, t12, t17, t71 * t9);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3(t74);
        let t77 = t76 * t76;
        let t79 = piecewise3(t75, t25, t77 * t74);
        let t80 = t79 * t31;
        let t81 = rho1 * rho1;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / t81;
        let t89 = t37 * sigma2;
        let t93 = f64::exp(-t47 * t89 * t85 / 24.0);
        let t94 = lapl1 * lapl1;
        let t95 = t56 * t94;
        let t96 = t81 * rho1;
        let t98 = 1.0 / t82 / t96;
        let t102 = 5.0 / 72.0 * t38 * sigma2 * t85 + t93 + t54 * t95 * t98 / 576.0;
        let t106 = piecewise3(t70, 0.0, 3.0 / 20.0 * t7 * t80 * t102);
        let tzk0 = t69 + t106;
        zk[ip] += tzk0;
        let t107 = t8 * t8;
        let t108 = 1.0 / t107;
        let t109 = t18 * t108;
        let t111 = piecewise5(t12, 0.0, t16, 0.0, t9 - t109);
        let t114 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t111);
        let t115 = t114 * t31;
        let t119 = 1.0 / t30;
        let t120 = t29 * t119;
        let t123 = t7 * t120 * t65 / 10.0;
        let t125 = 1.0 / t41 / t59;
        let t126 = sigma0 * t125;
        let t129 = t47 * t37;
        let t133 = t39 * t39;
        let t135 = 1.0 / t40 / t133;
        let t139 = -5.0 / 27.0 * t38 * t126 + t129 * t126 * t52 / 9.0 - 5.0 / 864.0 * t54 * t58 * t135;
        let t144 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t115 * t65 + t123 + 3.0 / 20.0 * t7 * t32 * t139);
        let t145 = t71 * t108;
        let t147 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t145);
        let t150 = piecewise3(t75, 0.0, 5.0 / 3.0 * t77 * t147);
        let t151 = t150 * t31;
        let t155 = t79 * t119;
        let t158 = t7 * t155 * t102 / 10.0;
        let t160 = piecewise3(t70, 0.0, 3.0 / 20.0 * t7 * t151 * t102 + t158);
        let tvrho0 = t69 + t106 + t8 * (t144 + t160);
        vrho[ip * 2] += tvrho0;
        let t164 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t109);
        let t167 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t164);
        let t168 = t167 * t31;
        let t173 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t168 * t65 + t123);
        let t175 = piecewise5(t16, 0.0, t12, 0.0, t9 - t145);
        let t178 = piecewise3(t75, 0.0, 5.0 / 3.0 * t77 * t175);
        let t179 = t178 * t31;
        let t184 = 1.0 / t83 / t96;
        let t185 = sigma2 * t184;
        let t191 = t81 * t81;
        let t193 = 1.0 / t82 / t191;
        let t197 = -5.0 / 27.0 * t38 * t185 + t129 * t185 * t93 / 9.0 - 5.0 / 864.0 * t54 * t95 * t193;
        let t202 = piecewise3(t70, 0.0, 3.0 / 20.0 * t7 * t179 * t102 + t158 + 3.0 / 20.0 * t7 * t80 * t197);
        let tvrho1 = t69 + t106 + t8 * (t173 + t202);
        vrho[ip * 2 + 1] += tvrho1;
        let t211 = 5.0 / 72.0 * t38 * t43 - t47 * t37 * t43 * t52 / 24.0;
        let t215 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t211);
        let tvsigma0 = t8 * t215;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t222 = 5.0 / 72.0 * t38 * t85 - t47 * t37 * t85 * t93 / 24.0;
        let t226 = piecewise3(t70, 0.0, 3.0 / 20.0 * t7 * t80 * t222);
        let tvsigma2 = t8 * t226;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t227 = t7 * t32;
        let t228 = t56 * lapl0;
        let t230 = t54 * t228 * t61;
        let t233 = piecewise3(t2, 0.0, t227 * t230 / 1920.0);
        let tvlapl0 = t8 * t233;
        vlapl[ip * 2] += tvlapl0;
        let t234 = t7 * t80;
        let t235 = t56 * lapl1;
        let t237 = t54 * t235 * t98;
        let t240 = piecewise3(t70, 0.0, t234 * t237 / 1920.0);
        let tvlapl1 = t8 * t240;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
