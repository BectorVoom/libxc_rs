//! GGA_X_S12 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_s12_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
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
        let t26 = t5 * t25;
        let t27 = pow_1_3::<f64>(t6);
        let t28 = t27 * param_bx;
        let t29 = param_C * sigma0;
        let t30 = rho0 * rho0;
        let t31 = pow_1_3::<f64>(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / t30;
        let t36 = sigma0 * sigma0;
        let t37 = param_D * t36;
        let t38 = t30 * t30;
        let t39 = t38 * rho0;
        let t41 = 1.0 / t31 / t39;
        let t43 = t29 * t34 + t37 * t41 + 1.0;
        let t46 = param_B * (1.0 - 1.0 / t43);
        let t47 = param_E * sigma0;
        let t49 = t47 * t34 + 1.0;
        let t51 = 1.0 - 1.0 / t49;
        let t53 = t46 * t51 + param_A;
        let t54 = t28 * t53;
        let t57 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t26 * t54);
        let t58 = rho1 <= dens_threshold;
        let t59 = -t16;
        let t61 = piecewise5::<f64>(t14, t11, t10, t15, t59 * t7);
        let t62 = 1.0 + t61;
        let t63 = t62 <= zeta_threshold;
        let t64 = pow_1_3::<f64>(t62);
        let t66 = piecewise3::<f64>(t63, t22, t64 * t62);
        let t67 = t5 * t66;
        let t68 = param_C * sigma2;
        let t69 = rho1 * rho1;
        let t70 = pow_1_3::<f64>(rho1);
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t69;
        let t75 = sigma2 * sigma2;
        let t76 = param_D * t75;
        let t77 = t69 * t69;
        let t78 = t77 * rho1;
        let t80 = 1.0 / t70 / t78;
        let t82 = t68 * t73 + t76 * t80 + 1.0;
        let t85 = param_B * (1.0 - 1.0 / t82);
        let t86 = param_E * sigma2;
        let t88 = t86 * t73 + 1.0;
        let t90 = 1.0 - 1.0 / t88;
        let t92 = t85 * t90 + param_A;
        let t93 = t28 * t92;
        let t96 = piecewise3::<f64>(t58, 0.0, -3.0 / 8.0 * t67 * t93);
        let tzk0 = t57 + t96;
        zk[ip] += tzk0;
        let t97 = t6 * t6;
        let t98 = 1.0 / t97;
        let t99 = t16 * t98;
        let t101 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t99);
        let t104 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t101);
        let t105 = t5 * t104;
        let t108 = t27 * t27;
        let t110 = 1.0 / t108 * param_bx;
        let t111 = t110 * t53;
        let t113 = t26 * t111 / 8.0;
        let t114 = t43 * t43;
        let t116 = param_B / t114;
        let t117 = t30 * rho0;
        let t119 = 1.0 / t32 / t117;
        let t122 = t38 * t30;
        let t124 = 1.0 / t31 / t122;
        let t127 = -8.0 / 3.0 * t29 * t119 - 16.0 / 3.0 * t37 * t124;
        let t128 = t127 * t51;
        let t130 = t49 * t49;
        let t131 = 1.0 / t130;
        let t132 = t46 * t131;
        let t136 = t116 * t128 - 8.0 / 3.0 * t132 * t47 * t119;
        let t137 = t28 * t136;
        let t141 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t105 * t54 - t113 - 3.0 / 8.0 * t26 * t137);
        let t142 = t59 * t98;
        let t144 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t142);
        let t147 = piecewise3::<f64>(t63, 0.0, 4.0 / 3.0 * t64 * t144);
        let t148 = t5 * t147;
        let t151 = t110 * t92;
        let t153 = t67 * t151 / 8.0;
        let t155 = piecewise3::<f64>(t58, 0.0, -3.0 / 8.0 * t148 * t93 - t153);
        let tvrho0 = t57 + t96 + t6 * (t141 + t155);
        vrho[ip * 2] += tvrho0;
        let t159 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t99);
        let t162 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t159);
        let t163 = t5 * t162;
        let t167 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t163 * t54 - t113);
        let t169 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t142);
        let t172 = piecewise3::<f64>(t63, 0.0, 4.0 / 3.0 * t64 * t169);
        let t173 = t5 * t172;
        let t176 = t82 * t82;
        let t178 = param_B / t176;
        let t179 = t69 * rho1;
        let t181 = 1.0 / t71 / t179;
        let t184 = t77 * t69;
        let t186 = 1.0 / t70 / t184;
        let t189 = -8.0 / 3.0 * t68 * t181 - 16.0 / 3.0 * t76 * t186;
        let t190 = t189 * t90;
        let t192 = t88 * t88;
        let t193 = 1.0 / t192;
        let t194 = t85 * t193;
        let t198 = t178 * t190 - 8.0 / 3.0 * t194 * t86 * t181;
        let t199 = t28 * t198;
        let t203 = piecewise3::<f64>(t58, 0.0, -3.0 / 8.0 * t173 * t93 - t153 - 3.0 / 8.0 * t67 * t199);
        let tvrho1 = t57 + t96 + t6 * (t167 + t203);
        vrho[ip * 2 + 1] += tvrho1;
        let t207 = param_D * sigma0;
        let t210 = 2.0 * t207 * t41 + param_C * t34;
        let t211 = t210 * t51;
        let t213 = t131 * param_E;
        let t214 = t213 * t34;
        let t216 = t116 * t211 + t46 * t214;
        let t217 = t28 * t216;
        let t220 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t26 * t217);
        let tvsigma0 = t6 * t220;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t222 = param_D * sigma2;
        let t225 = 2.0 * t222 * t80 + param_C * t73;
        let t226 = t225 * t90;
        let t228 = t193 * param_E;
        let t229 = t228 * t73;
        let t231 = t178 * t226 + t85 * t229;
        let t232 = t28 * t231;
        let t235 = piecewise3::<f64>(t58, 0.0, -3.0 / 8.0 * t67 * t232);
        let tvsigma2 = t6 * t235;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
