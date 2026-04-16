//! MGGA_K_GEA4 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 74 shared lines across all orders.
//! Delta: 58 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_gea4_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
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
        // --- shared preamble (74 lines) ---
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
        let t38 = t33 / t36;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t48 = 1.0 / t41 / rho0;
        let t52 = t33 * t33;
        let t54 = 1.0 / t35 / t34;
        let t55 = t52 * t54;
        let t56 = lapl0 * lapl0;
        let t57 = t39 * rho0;
        let t59 = 1.0 / t40 / t57;
        let t63 = t39 * t39;
        let t65 = 1.0 / t40 / t63;
        let t66 = sigma0 * t65;
        let t70 = sigma0 * sigma0;
        let t71 = t63 * rho0;
        let t73 = 1.0 / t40 / t71;
        let t77 = 1.0 + 5.0 / 648.0 * t38 * sigma0 * t43 + 5.0 / 54.0 * t38 * lapl0 * t48 + t55 * t56 * t59 / 5832.0 - t55 * t66 * lapl0 / 5184.0 + t55 * t70 * t73 / 17496.0;
        let t81 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t77);
        let t82 = rho1 <= dens_threshold;
        let t83 = -t18;
        let t85 = piecewise5(t16, t13, t12, t17, t83 * t9);
        let t86 = 1.0 + t85;
        let t87 = t86 <= zeta_threshold;
        let t88 = pow_1_3(t86);
        let t89 = t88 * t88;
        let t91 = piecewise3(t87, t25, t89 * t86);
        let t92 = t91 * t31;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t102 = 1.0 / t95 / rho1;
        let t106 = lapl1 * lapl1;
        let t107 = t93 * rho1;
        let t109 = 1.0 / t94 / t107;
        let t113 = t93 * t93;
        let t115 = 1.0 / t94 / t113;
        let t116 = sigma2 * t115;
        let t120 = sigma2 * sigma2;
        let t121 = t113 * rho1;
        let t123 = 1.0 / t94 / t121;
        let t127 = 1.0 + 5.0 / 648.0 * t38 * sigma2 * t97 + 5.0 / 54.0 * t38 * lapl1 * t102 + t55 * t106 * t109 / 5832.0 - t55 * t116 * lapl1 / 5184.0 + t55 * t120 * t123 / 17496.0;
        let t131 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t92 * t127);
        let tzk0 = t81 + t131;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (58 lines) ---
        let t132 = t8 * t8;
        let t133 = 1.0 / t132;
        let t134 = t18 * t133;
        let t136 = piecewise5(t12, 0.0, t16, 0.0, t9 - t134);
        let t139 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t136);
        let t140 = t139 * t31;
        let t144 = 1.0 / t30;
        let t145 = t29 * t144;
        let t148 = t7 * t145 * t77 / 10.0;
        let t150 = 1.0 / t41 / t57;
        let t160 = sigma0 * t73;
        let t164 = t63 * t39;
        let t166 = 1.0 / t40 / t164;
        let t170 = -5.0 / 243.0 * t38 * sigma0 * t150 - 25.0 / 162.0 * t38 * lapl0 * t43 - 5.0 / 8748.0 * t55 * t56 * t65 + 13.0 / 15552.0 * t55 * t160 * lapl0 - 2.0 / 6561.0 * t55 * t70 * t166;
        let t175 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t140 * t77 + t148 + 3.0 / 20.0 * t7 * t32 * t170);
        let t176 = t83 * t133;
        let t178 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t176);
        let t181 = piecewise3(t87, 0.0, 5.0 / 3.0 * t89 * t178);
        let t182 = t181 * t31;
        let t186 = t91 * t144;
        let t189 = t7 * t186 * t127 / 10.0;
        let t191 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t182 * t127 + t189);
        let tvrho0 = t81 + t131 + t8 * (t175 + t191);
        vrho[ip * 2] += tvrho0;
        let t195 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t134);
        let t198 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t195);
        let t199 = t198 * t31;
        let t204 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t199 * t77 + t148);
        let t206 = piecewise5(t16, 0.0, t12, 0.0, t9 - t176);
        let t209 = piecewise3(t87, 0.0, 5.0 / 3.0 * t89 * t206);
        let t210 = t209 * t31;
        let t215 = 1.0 / t95 / t107;
        let t225 = sigma2 * t123;
        let t229 = t113 * t93;
        let t231 = 1.0 / t94 / t229;
        let t235 = -5.0 / 243.0 * t38 * sigma2 * t215 - 25.0 / 162.0 * t38 * lapl1 * t97 - 5.0 / 8748.0 * t55 * t106 * t115 + 13.0 / 15552.0 * t55 * t225 * lapl1 - 2.0 / 6561.0 * t55 * t120 * t231;
        let t240 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t210 * t127 + t189 + 3.0 / 20.0 * t7 * t92 * t235);
        let tvrho1 = t81 + t131 + t8 * (t204 + t240);
        vrho[ip * 2 + 1] += tvrho1;
        let t243 = t38 * t43;
        let t246 = t55 * t65 * lapl0;
        let t248 = t55 * t160;
        let t250 = 5.0 / 648.0 * t243 - t246 / 5184.0 + t248 / 8748.0;
        let t254 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t250);
        let tvsigma0 = t8 * t254;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t255 = t38 * t97;
        let t258 = t55 * t115 * lapl1;
        let t260 = t55 * t225;
        let t262 = 5.0 / 648.0 * t255 - t258 / 5184.0 + t260 / 8748.0;
        let t266 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t92 * t262);
        let tvsigma2 = t8 * t266;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t274 = 5.0 / 54.0 * t38 * t48 + t55 * lapl0 * t59 / 2916.0 - t55 * t66 / 5184.0;
        let t278 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t274);
        let tvlapl0 = t8 * t278;
        vlapl[ip * 2] += tvlapl0;
        let t286 = 5.0 / 54.0 * t38 * t102 + t55 * lapl1 * t109 / 2916.0 - t55 * t116 / 5184.0;
        let t290 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t92 * t286);
        let tvlapl1 = t8 * t290;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
