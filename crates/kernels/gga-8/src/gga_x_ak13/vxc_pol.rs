//! GGA_X_AK13 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 57 shared lines across all orders.
//! Delta: 68 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_ak13_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_B1: f64,
    param_B2: f64,
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
        // --- shared preamble (57 lines) ---
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
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = 1.0 / t32;
        let t34 = param_B1 * t29 * t33;
        let t35 = f64::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t39 = t35 * t38;
        let t40 = t29 * t33;
        let t43 = 1.0 + t40 * t39 / 12.0;
        let t44 = f64::ln(t43);
        let t49 = param_B2 * t29 * t33;
        let t50 = 1.0 + t44;
        let t51 = f64::ln(t50);
        let t55 = 1.0 + t34 * t39 * t44 / 12.0 + t49 * t39 * t51 / 12.0;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = f64::sqrt(sigma2);
        let t71 = pow_1_3(rho1);
        let t73 = 1.0 / t71 / rho1;
        let t74 = t70 * t73;
        let t77 = 1.0 + t40 * t74 / 12.0;
        let t78 = f64::ln(t77);
        let t82 = 1.0 + t78;
        let t83 = f64::ln(t82);
        let t87 = 1.0 + t34 * t74 * t78 / 12.0 + t49 * t74 * t83 / 12.0;
        let t91 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t87);
        let tzk0 = t59 + t91;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (68 lines) ---
        let t92 = t6 * t6;
        let t93 = 1.0 / t92;
        let t94 = t16 * t93;
        let t96 = piecewise5(t10, 0.0, t14, 0.0, t7 - t94);
        let t99 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t96);
        let t100 = t99 * t26;
        let t104 = t26 * t26;
        let t105 = 1.0 / t104;
        let t106 = t25 * t105;
        let t109 = t5 * t106 * t55 / 8.0;
        let t110 = rho0 * rho0;
        let t112 = 1.0 / t36 / t110;
        let t113 = t35 * t112;
        let t117 = param_B1 * t28;
        let t118 = t32 * t32;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t121 = t110 * rho0;
        let t122 = t36 * t36;
        let t124 = 1.0 / t122 / t121;
        let t125 = sigma0 * t124;
        let t126 = 1.0 / t43;
        let t134 = param_B2 * t28 * t119;
        let t135 = 1.0 / t50;
        let t136 = t126 * t135;
        let t140 = -t34 * t113 * t44 / 9.0 - t120 * t125 * t126 / 18.0 - t49 * t113 * t51 / 9.0 - t134 * t125 * t136 / 18.0;
        let t145 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t100 * t55 - t109 - 3.0 / 8.0 * t5 * t27 * t140);
        let t146 = t61 * t93;
        let t148 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t146);
        let t151 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t148);
        let t152 = t151 * t26;
        let t156 = t68 * t105;
        let t159 = t5 * t156 * t87 / 8.0;
        let t161 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t152 * t87 - t159);
        let tvrho0 = t59 + t91 + t6 * (t145 + t161);
        vrho[ip * 2] += tvrho0;
        let t165 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t94);
        let t168 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t165);
        let t169 = t168 * t26;
        let t174 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t169 * t55 - t109);
        let t176 = piecewise5(t14, 0.0, t10, 0.0, t7 - t146);
        let t179 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t176);
        let t180 = t179 * t26;
        let t184 = rho1 * rho1;
        let t186 = 1.0 / t71 / t184;
        let t187 = t70 * t186;
        let t191 = t184 * rho1;
        let t192 = t71 * t71;
        let t194 = 1.0 / t192 / t191;
        let t195 = sigma2 * t194;
        let t196 = 1.0 / t77;
        let t203 = 1.0 / t82;
        let t204 = t196 * t203;
        let t208 = -t34 * t187 * t78 / 9.0 - t120 * t195 * t196 / 18.0 - t49 * t187 * t83 / 9.0 - t134 * t195 * t204 / 18.0;
        let t213 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t180 * t87 - t159 - 3.0 / 8.0 * t5 * t69 * t208);
        let tvrho1 = t59 + t91 + t6 * (t174 + t213);
        vrho[ip * 2 + 1] += tvrho1;
        let t216 = 1.0 / t35;
        let t217 = t216 * t38;
        let t222 = 1.0 / t122 / t110;
        let t234 = t34 * t217 * t44 / 24.0 + t117 * t119 * t222 * t126 / 48.0 + t49 * t217 * t51 / 24.0 + t134 * t222 * t126 * t135 / 48.0;
        let t238 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t234);
        let tvsigma0 = t6 * t238;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t239 = 1.0 / t70;
        let t240 = t239 * t73;
        let t245 = 1.0 / t192 / t184;
        let t257 = t34 * t240 * t78 / 24.0 + t117 * t119 * t245 * t196 / 48.0 + t49 * t240 * t83 / 24.0 + t134 * t245 * t196 * t203 / 48.0;
        let t261 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t257);
        let tvsigma2 = t6 * t261;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
