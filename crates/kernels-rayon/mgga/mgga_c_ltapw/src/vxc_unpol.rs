//! MGGA_C_LTAPW vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ltapw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ltapw_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t25 = rmath::pow(5.0 / 9.0 * tau[ip] * t9 / t12 / rho[ip] * t15 / t19, 3.0 / 5.0 * param_ltafrac);
        let t26 = rho[ip] * t25;
        let t27 = pow_1_3(t26);
        let t30 = t5 * t7 / t27;
        let t32 = 1.0 + 0.053425 * t30;
        let t33 = rmath::sqrt(t30);
        let t36 = pow_3_2(t30);
        let t38 = t2 * t2;
        let t39 = t4 * t4;
        let t40 = t38 * t39;
        let t41 = t27 * t27;
        let t44 = t40 * t6 / t41;
        let t46 = 3.79785 * t33 + 0.8969 * t30 + 0.204775 * t36 + 0.123235 * t44;
        let t49 = 1.0 + 16.081824322151103 / t46;
        let t50 = rmath::ln(t49);
        let t52 = 0.062182 * t32 * t50;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t62 = (2.0 * t56 - 2.0) / (2.0 * t8 - 2.0);
        let t64 = 1.0 + 0.0278125 * t30;
        let t69 = 5.1785 * t33 + 0.905775 * t30 + 0.1100325 * t36 + 0.1241775 * t44;
        let t72 = 1.0 + 29.608574643216677 / t69;
        let t73 = rmath::ln(t72);
        let t76 = 0.019751789702565206 * t62 * t64 * t73;
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
        let t98 = rmath::sqrt(t30);
        let t99 = t98 * t2;
        let t100 = t99 * t4;
        let t101 = t100 * t93;
        let t104 = 1.0 / t41 / t26;
        let t105 = t6 * t104;
        let t107 = t40 * t105 * t81;
        let t109 = -0.632975 * t94 - 0.29896666666666666 * t96 - 0.1023875 * t101 - 0.08215666666666667 * t107;
        let t110 = 1.0 / t49;
        let t111 = t109 * t110;
        let t112 = t88 * t111;
        let t114 = t62 * t5;
        let t115 = t81 * t73;
        let t117 = t114 * t92 * t115;
        let t119 = t62 * t64;
        let t120 = t69 * t69;
        let t121 = 1.0 / t120;
        let t126 = -0.8630833333333333 * t94 - 0.301925 * t96 - 0.05501625 * t101 - 0.082785 * t107;
        let t127 = t121 * t126;
        let t128 = 1.0 / t72;
        let t129 = t127 * t128;
        let t130 = t119 * t129;
        let tvrho0 = -t52 + t76 + rho[ip] * (0.0011073577833333333 * t84 + 1.0 * t112 - 0.0001831155503675316 * t117 - 0.5848223397455204 * t130);
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t134 = t5 * t92;
        let t135 = 1.0 / tau[ip];
        let t136 = param_ltafrac * t135;
        let t140 = 0.00066441467 * t134 * t26 * t136 * t50;
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
        let t157 = -0.379785 * t146 - 0.17938 * t149 - 0.0614325 * t152 - 0.049294 * t155;
        let t158 = t157 * t110;
        let t160 = 1.0 * t88 * t158;
        let t161 = t62 * t77;
        let t162 = t143 * t25;
        let t163 = t136 * t73;
        let t166 = 0.00010986933022051895 * t161 * t162 * t163;
        let t171 = -0.51785 * t146 - 0.181155 * t149 - 0.03300975 * t152 - 0.049671 * t155;
        let t172 = t121 * t171;
        let t173 = t172 * t128;
        let t175 = 0.5848223397455204 * t119 * t173;
        let tvtau0 = rho[ip] * (t140 + t160 - t166 - t175);
        vtau[ip] += tvtau0;
    }
}
