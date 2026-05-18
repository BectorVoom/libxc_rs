//! GGA_X_BEEFVDW exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_beefvdw_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t19 = t18 + 1.0;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3::<f64>(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3::<f64>(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t42 = 4.0 + t33 * t39 / 24.0;
        let t43 = 1.0 / t42;
        let t45 = t33 * t39 * t43;
        let t47 = t45 / 12.0 - 1.0;
        let t48 = t47 * t47;
        let t49 = t48 * t48;
        let t51 = t48 * t47;
        let t54 = t49 * t49;
        let t55 = t54 * t54;
        let t57 = t49 * t47;
        let t58 = t54 * t57;
        let t60 = t49 * t48;
        let t61 = t54 * t60;
        let t63 = t54 * t51;
        let t65 = t54 * t49;
        let t68 = t49 * t51;
        let t71 = t55 * t49;
        let t73 = t55 * t57;
        let t75 = t55 * t60;
        let t77 = t55 * t54;
        let t79 = -0.69459735177638985466e0 * t49 + 0.52755620115589800943e0 * t51 - 0.38916037779196815969e0 * t48 - 0.16837084139014120539e6 * t55 - 0.281024018056846299e4 * t58 + 0.70504541869034010051e5 * t61 + 0.22748997850816485208e4 * t63 - 0.20148245175625047025e5 * t65 - 0.44233229018433803622e3 * t54 + 0.86005730499279641299e2 * t68 + 0.30542034959315850168e2 * t60 - 0.32352403136049329184e6 * t71 + 0.18078200670879145336e6 * t73 + 0.2558947952623533461e6 * t75 - 0.13204466182182150467e6 * t77;
        let t80 = t55 * t68;
        let t82 = t54 * t47;
        let t83 = t55 * t82;
        let t85 = t55 * t65;
        let t87 = t54 * t48;
        let t88 = t55 * t87;
        let t90 = t55 * t63;
        let t94 = t55 * t51;
        let t96 = t55 * t47;
        let t98 = t55 * t48;
        let t104 = t54 * t68;
        let t106 = 0.11313514630621233134e1 - 0.16114215399846280595e6 * t80 + 0.90365611108522808258e5 * t83 - 0.5427777462637186032e4 * t85 + 0.40074935854432390114e5 * t88 - 0.29150193011493262292e5 * t90 + 0.4135586188014653875e4 * t55 * t58 - 0.12981481812794983922e6 * t94 + 0.56174007979372666951e5 * t96 + 0.27967048856303053872e6 * t98 + 0.37835396407252402359e4 * t87 - 0.72975787893717136018e1 * t57 - 0.61754786104528599731e3 * t82 + 0.37534251004296526981e-1 * t45 - 0.10276426607863824397e5 * t104;
        let t107 = t79 + t106;
        let t111 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t107);
        let t112 = rho1 <= dens_threshold;
        let t113 = -t16;
        let t115 = piecewise5::<f64>(t14, t11, t10, t15, t113 * t7);
        let t116 = t115 + 1.0;
        let t117 = t116 <= zeta_threshold;
        let t118 = pow_1_3::<f64>(t116);
        let t120 = piecewise3::<f64>(t117, t22, t118 * t116);
        let t121 = t120 * t26;
        let t122 = rho1 * rho1;
        let t123 = pow_1_3::<f64>(rho1);
        let t124 = t123 * t123;
        let t126 = 1.0 / t124 / t122;
        let t127 = sigma2 * t126;
        let t130 = 4.0 + t33 * t127 / 24.0;
        let t131 = 1.0 / t130;
        let t133 = t33 * t127 * t131;
        let t135 = t133 / 12.0 - 1.0;
        let t136 = t135 * t135;
        let t137 = t136 * t135;
        let t138 = t136 * t136;
        let t139 = t138 * t137;
        let t140 = t138 * t138;
        let t141 = t140 * t139;
        let t143 = t140 * t138;
        let t145 = t138 * t135;
        let t146 = t140 * t145;
        let t148 = t140 * t137;
        let t152 = t140 * t135;
        let t155 = t140 * t140;
        let t156 = t155 * t148;
        let t160 = t138 * t136;
        let t163 = t155 * t140;
        let t165 = t155 * t152;
        let t167 = -0.10276426607863824397e5 * t141 - 0.20148245175625047025e5 * t143 - 0.281024018056846299e4 * t146 + 0.22748997850816485208e4 * t148 + 0.86005730499279641299e2 * t139 - 0.44233229018433803622e3 * t140 - 0.61754786104528599731e3 * t152 + 0.52755620115589800943e0 * t137 - 0.29150193011493262292e5 * t156 - 0.38916037779196815969e0 * t136 - 0.72975787893717136018e1 * t145 + 0.30542034959315850168e2 * t160 - 0.69459735177638985466e0 * t138 - 0.13204466182182150467e6 * t163 + 0.90365611108522808258e5 * t165;
        let t169 = t155 * t135;
        let t171 = t140 * t160;
        let t175 = t155 * t143;
        let t177 = t155 * t139;
        let t179 = t155 * t145;
        let t181 = t155 * t160;
        let t183 = t155 * t138;
        let t185 = t155 * t136;
        let t187 = t155 * t137;
        let t189 = t140 * t136;
        let t192 = t155 * t189;
        let t194 = 0.11313514630621233134e1 - 0.16837084139014120539e6 * t155 + 0.56174007979372666951e5 * t169 + 0.70504541869034010051e5 * t171 + 0.4135586188014653875e4 * t155 * t146 - 0.5427777462637186032e4 * t175 - 0.16114215399846280595e6 * t177 + 0.18078200670879145336e6 * t179 + 0.2558947952623533461e6 * t181 - 0.32352403136049329184e6 * t183 + 0.27967048856303053872e6 * t185 - 0.12981481812794983922e6 * t187 + 0.37835396407252402359e4 * t189 + 0.37534251004296526981e-1 * t133 + 0.40074935854432390114e5 * t192;
        let t195 = t167 + t194;
        let t199 = piecewise3::<f64>(t112, 0.0, -3.0 / 8.0 * t5 * t121 * t195);
        let tzk0 = t111 + t199;
        zk[ip] += tzk0;
    }
}
