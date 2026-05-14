//! LDA_C_2D_AMGB vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_2D_AMGB vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_2d_amgb_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = f64::sqrt(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 / t1;
        let t8 = 1.0 / t2 / t1;
        let t10 = 0.04869723403850762 * t3 + 0.018219548589342285 * t5 + 0.000603947002028882 * t8;
        let t12 = f64::sqrt(M_PI);
        let t13 = 1.0 / t12;
        let t14 = t13 * t3;
        let t15 = pow_3_2(t14);
        let t19 = 0.5654308006315614 * t3 - 0.02069 * t15 + 0.10821581200590331 * t5 + 0.00313738702352666 * t8;
        let t21 = 1.0 + 1.0 / t19;
        let t22 = f64::ln(t21);
        let t23 = t10 * t22;
        let t27 = -0.01914859446561085 * t3 - 0.0024406887987971425 * t5 - 1.643337945467037e-05 * t8;
        let t31 = 0.2331795548802877 * t3 + 0.021277965468762 * t5 + 0.0001400599965454174 * t8;
        let t33 = 1.0 + 1.0 / t31;
        let t34 = f64::ln(t33);
        let t36 = 0.117331 + t27 * t34;
        let t37 = rho0 - rho1;
        let t38 = t37 * t37;
        let t39 = t36 * t38;
        let t40 = t1 * t1;
        let t41 = 1.0 / t40;
        let t42 = t39 * t41;
        let t46 = -0.020927484222536923 * t3 + 0.005208122695761946 * t5 - 0.0048916627893863685 * t8;
        let t49 = 0.8035757880366529 * t3 + 0.2088776021566591 * t8;
        let t51 = 1.0 + 1.0 / t49;
        let t52 = f64::ln(t51);
        let t54 = 0.0234188 + t46 * t52;
        let t55 = t38 * t38;
        let t56 = t54 * t55;
        let t57 = t40 * t40;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t61 = f64::exp(-0.7552241765370266 * t3);
        let t63 = M_SQRT2;
        let t64 = (t61 - 1.0) * t63;
        let t65 = t13 * t2;
        let t66 = t37 * t5;
        let t67 = 1.0 + t66;
        let t68 = t67 <= zeta_threshold;
        let t69 = f64::sqrt(zeta_threshold);
        let t70 = t69 * zeta_threshold;
        let t71 = f64::sqrt(t67);
        let t72 = t71 * t67;
        let t73 = piecewise3(t68, t70, t72);
        let t75 = 1.0 - t66;
        let t76 = t75 <= zeta_threshold;
        let t77 = f64::sqrt(t75);
        let t78 = t77 * t75;
        let t79 = piecewise3(t76, t70, t78);
        let t85 = t73 / 2.0 + t79 / 2.0 - 1.0 - 3.0 / 8.0 * t38 * t41 - 3.0 / 128.0 * t55 * t58;
        let t88 = 4.0 / 3.0 * t64 * t65 * t85;
        let tzk0 = -0.1925 + t23 + t42 + t59 - t88;
        zk[ip] += tzk0;
        let t92 = 1.0 / t2 / t40;
        let t94 = -0.02434861701925381 * t8 - 0.018219548589342285 * t41 - 0.000905920503043323 * t92;
        let t95 = t94 * t22;
        let t96 = t19 * t19;
        let t97 = 1.0 / t96;
        let t98 = t10 * t97;
        let t100 = f64::sqrt(t14);
        let t101 = t100 * t13;
        let t106 = -0.2827154003157807 * t8 + 0.0155175 * t101 * t8 - 0.10821581200590331 * t41 - 0.00470608053528999 * t92;
        let t107 = 1.0 / t21;
        let t108 = t106 * t107;
        let t109 = t98 * t108;
        let t113 = 0.009574297232805425 * t8 + 0.0024406887987971425 * t41 + 2.4650069182005552e-05 * t92;
        let t115 = t31 * t31;
        let t116 = 1.0 / t115;
        let t117 = t27 * t116;
        let t121 = -0.11658977744014384 * t8 - 0.021277965468762 * t41 - 0.0002100899948181261 * t92;
        let t122 = 1.0 / t33;
        let t123 = t121 * t122;
        let t125 = t113 * t34 - t117 * t123;
        let t126 = t125 * t38;
        let t127 = t126 * t41;
        let t128 = t36 * t37;
        let t129 = t128 * t41;
        let t130 = 2.0 * t129;
        let t131 = t40 * t1;
        let t132 = 1.0 / t131;
        let t133 = t39 * t132;
        let t134 = 2.0 * t133;
        let t138 = 0.010463742111268461 * t8 - 0.005208122695761946 * t41 + 0.007337494184079552 * t92;
        let t140 = t49 * t49;
        let t141 = 1.0 / t140;
        let t142 = t46 * t141;
        let t145 = -0.40178789401832643 * t8 - 0.31331640323498866 * t92;
        let t146 = 1.0 / t51;
        let t147 = t145 * t146;
        let t149 = t138 * t52 - t142 * t147;
        let t150 = t149 * t55;
        let t151 = t150 * t58;
        let t152 = t38 * t37;
        let t153 = t54 * t152;
        let t154 = t153 * t58;
        let t155 = 4.0 * t154;
        let t156 = t57 * t1;
        let t157 = 1.0 / t156;
        let t158 = t56 * t157;
        let t159 = 4.0 * t158;
        let t160 = t5 * t61;
        let t161 = t63 * t85;
        let t162 = t160 * t161;
        let t163 = 0.2840597424304148 * t162;
        let t165 = t64 * t14 * t85;
        let t166 = 2.0 / 3.0 * t165;
        let t167 = t37 * t41;
        let t168 = t5 - t167;
        let t171 = piecewise3(t68, 0.0, 3.0 / 2.0 * t71 * t168);
        let t173 = -t168;
        let t176 = piecewise3(t76, 0.0, 3.0 / 2.0 * t77 * t173);
        let t178 = 3.0 / 4.0 * t167;
        let t180 = 3.0 / 4.0 * t38 * t132;
        let t182 = 3.0 / 32.0 * t152 * t58;
        let t184 = 3.0 / 32.0 * t55 * t157;
        let t185 = t171 / 2.0 + t176 / 2.0 - t178 + t180 - t182 + t184;
        let t187 = t64 * t65 * t185;
        let t188 = 4.0 / 3.0 * t187;
        let t189 = t95 - t109 + t127 + t130 - t134 + t151 + t155 - t159 - t163 - t166 - t188;
        let tvrho0 = -0.1925 + t23 + t42 + t59 - t88 + t1 * t189;
        vrho[ip * 2] += tvrho0;
        let t191 = -t5 - t167;
        let t194 = piecewise3(t68, 0.0, 3.0 / 2.0 * t71 * t191);
        let t196 = -t191;
        let t199 = piecewise3(t76, 0.0, 3.0 / 2.0 * t77 * t196);
        let t201 = t194 / 2.0 + t199 / 2.0 + t178 + t180 + t182 + t184;
        let t203 = t64 * t65 * t201;
        let t204 = 4.0 / 3.0 * t203;
        let t205 = t95 - t109 + t127 - t130 - t134 + t151 - t155 - t159 - t163 - t166 - t204;
        let tvrho1 = -0.1925 + t23 + t42 + t59 - t88 + t1 * t205;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
