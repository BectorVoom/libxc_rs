//! MGGA_C_LTAPW vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ltapw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_ltapw_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t11 = pow_1_3(rho[ip]);
        let t12 = t11 * t11;
        let t15 = M_CBRT6;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t25 = f64::powf(5.0 / 9.0 * tau[ip] * t9 / t12 / rho[ip] * t15 / t19, 3.0 / 5.0 * param_ltafrac);
        let t26 = rho[ip] * t25;
        let t27 = pow_1_3(t26);
        let t30 = t5 * t7 / t27;
        let t32 = 1.0 + 0.53425e-1 * t30;
        let t33 = f64::sqrt(t30);
        let t36 = pow_3_2(t30);
        let t38 = t2 * t2;
        let t39 = t4 * t4;
        let t40 = t38 * t39;
        let t41 = t27 * t27;
        let t44 = t40 * t6 / t41;
        let t46 = 0.379785e1 * t33 + 0.8969e0 * t30 + 0.204775e0 * t36 + 0.123235e0 * t44;
        let t49 = 1.0 + 0.16081824322151104822e2 / t46;
        let t50 = f64::ln(t49);
        let t52 = 0.62182e-1 * t32 * t50;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t62 = (2.0 * t56 - 2.0) / (2.0 * t8 - 2.0);
        let t64 = 1.0 + 0.278125e-1 * t30;
        let t69 = 0.51785e1 * t33 + 0.905775e0 * t30 + 0.1100325e0 * t36 + 0.1241775e0 * t44;
        let t72 = 1.0 + 0.29608574643216675549e2 / t69;
        let t73 = f64::ln(t72);
        let t76 = 0.19751789702565206229e-1 * t62 * t64 * t73;
        let tzk0 = -t52 + t76;
        zk[ip] += tzk0;
        let t77 = t5 * t7;
        let t79 = 1.0 / t27 / t26;
        let t80 = t25 * param_ltafrac;
        let t81 = t25 - t80;
        let t82 = t79 * t81;
        let t84 = t77 * t82 * t50;
        let t86 = t46 * t46;
        let t87 = 1.0 / t86;
        let t88 = t32 * t87;
        let t90 = 1.0 / t33 * t2;
        let t91 = t90 * t4;
        let t92 = t7 * t79;
        let t93 = t92 * t81;
        let t94 = t91 * t93;
        let t96 = t5 * t93;
        let t98 = f64::sqrt(t30);
        let t99 = t98 * t2;
        let t100 = t99 * t4;
        let t101 = t100 * t93;
        let t104 = 1.0 / t41 / t26;
        let t105 = t6 * t104;
        let t107 = t40 * t105 * t81;
        let t109 = -0.632975e0 * t94 - 0.29896666666666666667e0 * t96 - 0.1023875e0 * t101 - 0.82156666666666666667e-1 * t107;
        let t110 = 1.0 / t49;
        let t111 = t109 * t110;
        let t112 = t88 * t111;
        let t114 = t62 * t5;
        let t115 = t81 * t73;
        let t117 = t114 * t92 * t115;
        let t119 = t62 * t64;
        let t120 = t69 * t69;
        let t121 = 1.0 / t120;
        let t126 = -0.86308333333333333334e0 * t94 - 0.301925e0 * t96 - 0.5501625e-1 * t101 - 0.82785e-1 * t107;
        let t127 = t121 * t126;
        let t128 = 1.0 / t72;
        let t129 = t127 * t128;
        let t130 = t119 * t129;
        let tvrho0 = -t52 + t76 + rho[ip] * (0.11073577833333333333e-2 * t84 + 1.0 * t112 - 0.18311555036753159941e-3 * t117 - 0.58482233974552040708e0 * t130);
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t134 = t5 * t92;
        let t135 = 1.0 / tau[ip];
        let t136 = param_ltafrac * t135;
        let t140 = 0.66441467e-3 * t134 * t26 * t136 * t50;
        let t141 = t4 * t7;
        let t142 = t90 * t141;
        let t143 = t79 * rho[ip];
        let t144 = t80 * t135;
        let t145 = t143 * t144;
        let t146 = t142 * t145;
        let t148 = t26 * t136;
        let t149 = t134 * t148;
        let t151 = t99 * t141;
        let t152 = t151 * t145;
        let t154 = t40 * t105;
        let t155 = t154 * t148;
        let t157 = -0.379785e0 * t146 - 0.17938e0 * t149 - 0.614325e-1 * t152 - 0.49294e-1 * t155;
        let t158 = t157 * t110;
        let t160 = 1.0 * t88 * t158;
        let t161 = t62 * t77;
        let t162 = t143 * t25;
        let t163 = t136 * t73;
        let t166 = 0.10986933022051895965e-3 * t161 * t162 * t163;
        let t171 = -0.51785e0 * t146 - 0.181155e0 * t149 - 0.3300975e-1 * t152 - 0.49671e-1 * t155;
        let t172 = t121 * t171;
        let t173 = t172 * t128;
        let t175 = 0.58482233974552040708e0 * t119 * t173;
        let tvtau0 = rho[ip] * (t140 + t160 - t166 - t175);
        vtau[ip] += tvtau0;
    }
}
