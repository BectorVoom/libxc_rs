//! LDA_C_ML1 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_ml1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_C_ML1 vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_ml1_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = rho0 - rho1;
        let t3 = 1.0 / t1;
        let t4 = t2 * t3;
        let t5 = f64::abs(t4);
        let t7 = 1.0 - t5 <= zeta_threshold;
        let t8 = t2 * t2;
        let t9 = t1 * t1;
        let t10 = 1.0 / t9;
        let t12 = -t8 * t10 + 1.0;
        let t13 = pow_1_3::<f64>(t1);
        let t14 = t13 * param_fc;
        let t16 = 1.0 + t4 <= zeta_threshold;
        let t17 = zeta_threshold - 1.0;
        let t19 = 1.0 - t4 <= zeta_threshold;
        let t21 = piecewise5::<f64>(t16, t17, t19, -t17, t4);
        let t22 = 1.0 + t21;
        let t23 = f64::powf(t22, param_q);
        let t24 = 1.0 - t21;
        let t25 = f64::powf(t24, param_q);
        let t26 = t23 + t25;
        let t27 = t21 * t21;
        let t28 = 1.0 - t27;
        let t29 = pow_1_3::<f64>(t28);
        let t30 = t26 * t29;
        let t31 = pow_1_3::<f64>(t22);
        let t32 = pow_1_3::<f64>(t24);
        let t33 = t31 + t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t38 = 1.0 + 10.874334072525 * t14 * t35;
        let t41 = 1.0 / t13;
        let t42 = 1.0 / param_fc;
        let t43 = t41 * t42;
        let t44 = 1.0 / t26;
        let t45 = 1.0 / t29;
        let t46 = t44 * t45;
        let t47 = t46 * t33;
        let t48 = t43 * t47;
        let t50 = 1.0 + 0.09195962397381102 * t48;
        let t51 = f64::ln(t50);
        let t52 = t51 * t41;
        let t53 = t52 * t42;
        let t57 = t13 * t13;
        let t58 = 1.0 / t57;
        let t59 = param_fc * param_fc;
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t62 = t26 * t26;
        let t63 = 1.0 / t62;
        let t64 = t29 * t29;
        let t65 = 1.0 / t64;
        let t66 = t63 * t65;
        let t67 = t33 * t33;
        let t68 = t66 * t67;
        let t71 = -2.763169 / t38 + 0.28144540420067765 * t53 * t47 + 0.2541000285260132 * t48 - 0.049248579417833935 * t61 * t68;
        let t74 = piecewise3::<f64>(t7, 0.0, t12 * t71 / 4.0);
        let tzk0 = t1 * t74;
        zk[ip] += tzk0;
        let t75 = 2.0 * tzk0;
        let t76 = t2 * t10;
        let t77 = t9 * t1;
        let t78 = 1.0 / t77;
        let t79 = t8 * t78;
        let t81 = -2.0 * t76 + 2.0 * t79;
        let t83 = t38 * t38;
        let t84 = 1.0 / t83;
        let t85 = t58 * param_fc;
        let t87 = 3.624778024175 * t85 * t35;
        let t88 = t23 * param_q;
        let t90 = piecewise5::<f64>(t16, 0.0, t19, 0.0, t3 - t76);
        let t91 = 1.0 / t22;
        let t94 = t25 * param_q;
        let t95 = 1.0 / t24;
        let t98 = t88 * t90 * t91 - t94 * t90 * t95;
        let t100 = t98 * t29 * t34;
        let t103 = t14 * t26;
        let t104 = t65 * t34;
        let t105 = t21 * t90;
        let t106 = t104 * t105;
        let t109 = 1.0 / t67;
        let t110 = t29 * t109;
        let t111 = t31 * t31;
        let t112 = 1.0 / t111;
        let t114 = t32 * t32;
        let t115 = 1.0 / t114;
        let t118 = t112 * t90 / 3.0 - t115 * t90 / 3.0;
        let t119 = t110 * t118;
        let t122 = t87 + 10.874334072525 * t14 * t100 - 7.24955604835 * t103 * t106 - 10.874334072525 * t103 * t119;
        let t126 = 1.0 / t13 / t1;
        let t127 = t126 * t42;
        let t128 = t127 * t47;
        let t129 = 0.03065320799127034 * t128;
        let t130 = t43 * t63;
        let t131 = t45 * t33;
        let t132 = t131 * t98;
        let t133 = t130 * t132;
        let t135 = t43 * t44;
        let t137 = 1.0 / t29 / t28;
        let t138 = t137 * t33;
        let t139 = t138 * t105;
        let t140 = t135 * t139;
        let t142 = t46 * t118;
        let t143 = t43 * t142;
        let t145 = -t129 - 0.09195962397381102 * t133 + 0.06130641598254068 * t140 + 0.09195962397381102 * t143;
        let t146 = 1.0 / t50;
        let t147 = t145 * t146;
        let t148 = t147 * t41;
        let t149 = t42 * t44;
        let t150 = t149 * t131;
        let t153 = t51 * t126;
        let t154 = t153 * t42;
        let t156 = 0.09381513473355922 * t154 * t47;
        let t157 = t63 * t45;
        let t158 = t33 * t98;
        let t159 = t157 * t158;
        let t162 = t52 * t149;
        let t167 = 0.08470000950867107 * t128;
        let t172 = 1.0 / t57 / t1;
        let t173 = t172 * t60;
        let t175 = 0.032832386278555954 * t173 * t68;
        let t177 = 1.0 / t62 / t26;
        let t178 = t61 * t177;
        let t179 = t65 * t67;
        let t180 = t179 * t98;
        let t183 = t61 * t63;
        let t185 = 1.0 / t64 / t28;
        let t186 = t185 * t67;
        let t187 = t186 * t105;
        let t190 = t65 * t33;
        let t191 = t190 * t118;
        let t194 = 2.763169 * t84 * t122 + 0.28144540420067765 * t148 * t150 - t156 - 0.28144540420067765 * t53 * t159 + 0.18763026946711844 * t162 * t139 + 0.28144540420067765 * t53 * t142 - t167 - 0.2541000285260132 * t133 + 0.16940001901734214 * t140 + 0.2541000285260132 * t143 + t175 + 0.09849715883566787 * t178 * t180 - 0.06566477255711191 * t183 * t187 - 0.09849715883566787 * t183 * t191;
        let t198 = piecewise3::<f64>(t7, 0.0, t12 * t194 / 4.0 + t81 * t71 / 4.0);
        let tvrho0 = t9 * t198 + t75;
        vrho[ip * 2] += tvrho0;
        let t201 = 2.0 * t76 + 2.0 * t79;
        let t204 = piecewise5::<f64>(t16, 0.0, t19, 0.0, -t3 - t76);
        let t209 = t88 * t204 * t91 - t94 * t204 * t95;
        let t211 = t209 * t29 * t34;
        let t214 = t21 * t204;
        let t215 = t104 * t214;
        let t221 = t112 * t204 / 3.0 - t115 * t204 / 3.0;
        let t222 = t110 * t221;
        let t225 = t87 + 10.874334072525 * t14 * t211 - 7.24955604835 * t103 * t215 - 10.874334072525 * t103 * t222;
        let t228 = t131 * t209;
        let t229 = t130 * t228;
        let t231 = t138 * t214;
        let t232 = t135 * t231;
        let t234 = t46 * t221;
        let t235 = t43 * t234;
        let t237 = -t129 - 0.09195962397381102 * t229 + 0.06130641598254068 * t232 + 0.09195962397381102 * t235;
        let t238 = t237 * t146;
        let t239 = t238 * t41;
        let t242 = t33 * t209;
        let t243 = t157 * t242;
        let t253 = t179 * t209;
        let t256 = t186 * t214;
        let t259 = t190 * t221;
        let t262 = 2.763169 * t84 * t225 + 0.28144540420067765 * t239 * t150 - t156 - 0.28144540420067765 * t53 * t243 + 0.18763026946711844 * t162 * t231 + 0.28144540420067765 * t53 * t234 - t167 - 0.2541000285260132 * t229 + 0.16940001901734214 * t232 + 0.2541000285260132 * t235 + t175 + 0.09849715883566787 * t178 * t253 - 0.06566477255711191 * t183 * t256 - 0.09849715883566787 * t183 * t259;
        let t266 = piecewise3::<f64>(t7, 0.0, t12 * t262 / 4.0 + t201 * t71 / 4.0);
        let tvrho1 = t9 * t266 + t75;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
