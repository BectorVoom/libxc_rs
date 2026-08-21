//! LDA_C_1D_CSC kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_csc_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_para_4: f64,
    param_para_7: f64,
    param_para_9: f64,
    param_para_8: f64,
    param_para_1: f64,
    param_para_5: f64,
    param_para_2: f64,
    param_para_6: f64,
    param_para_3: f64,
    param_para_0: f64,
    param_ferro_4: f64,
    param_ferro_7: f64,
    param_ferro_9: f64,
    param_ferro_8: f64,
    param_ferro_1: f64,
    param_ferro_5: f64,
    param_ferro_2: f64,
    param_ferro_6: f64,
    param_ferro_3: f64,
    param_ferro_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 / rho[ip];
        let t2 = t1 / 2.0;
        let t3 = param_para_4;
        let t4 = rho[ip] * rho[ip];
        let t5 = 1.0 / t4;
        let t8 = t2 + t3 * t5 / 4.0;
        let t9 = param_para_7;
        let t13 = param_para_9;
        let t14 = rmath::pow(t2, t13);
        let t15 = param_para_8 * t14;
        let t16 = 1.0 + t9 * t1 / 2.0 + t15;
        let t17 = rmath::ln(t16);
        let t18 = t8 * t17;
        let t21 = param_para_1;
        let t24 = param_para_5;
        let t25 = rmath::pow(t2, t24);
        let t26 = param_para_2 * t25;
        let t29 = param_para_6;
        let t30 = rmath::pow(t2, t29);
        let t31 = param_para_3 * t30;
        let t33 = t21 * t1 + 2.0 * t26 + 2.0 * t31 + 2.0 * param_para_0;
        let t34 = 1.0 / t33;
        let tzk0 = -t18 * t34;
        zk[ip] += tzk0;
        let t37 = 1.0 / t4 / rho[ip];
        let t40 = -t3 * t37 / 2.0 - t5 / 2.0;
        let t41 = rho[ip] * t40;
        let t42 = t17 * t34;
        let t44 = rho[ip] * t8;
        let t49 = -t9 * t5 / 2.0 - t15 * t13 * t1;
        let t50 = 1.0 / t16;
        let t52 = t49 * t50 * t34;
        let t54 = t33 * t33;
        let t55 = 1.0 / t54;
        let t56 = t17 * t55;
        let t64 = -2.0 * t26 * t24 * t1 - 2.0 * t31 * t29 * t1 - t21 * t5;
        let t65 = t56 * t64;
        let tvrho0 = -t41 * t42 - t44 * t52 + t44 * t65 + tzk0;
        vrho[ip] += tvrho0;
        let t67 = t40 * t17;
        let t70 = t8 * t49;
        let t71 = t50 * t34;
        let t74 = t55 * t64;
        let t77 = t4 * t4;
        let t78 = 1.0 / t77;
        let t81 = t37 + 3.0 / 2.0 * t3 * t78;
        let t82 = rho[ip] * t81;
        let t89 = t13 * t13;
        let t94 = t15 * t13 * t5 + t15 * t89 * t5 + t9 * t37;
        let t96 = t94 * t50 * t34;
        let t98 = t49 * t49;
        let t99 = t16 * t16;
        let t100 = 1.0 / t99;
        let t102 = t98 * t100 * t34;
        let t104 = t44 * t49;
        let t105 = t50 * t55;
        let t106 = t105 * t64;
        let t110 = 1.0 / t54 / t33;
        let t111 = t17 * t110;
        let t112 = t64 * t64;
        let t113 = t111 * t112;
        let t117 = t24 * t24;
        let t122 = t29 * t29;
        let t128 = 2.0 * t26 * t117 * t5 + 2.0 * t31 * t122 * t5 + 2.0 * t26 * t24 * t5 + 2.0 * t31 * t29 * t5 + 2.0 * t21 * t37;
        let t129 = t56 * t128;
        let tv2rho20 = t44 * t102 + 2.0 * t104 * t106 - 2.0 * t44 * t113 + t44 * t129 + 2.0 * t18 * t74 - 2.0 * t67 * t34 - 2.0 * t41 * t52 + 2.0 * t41 * t65 - t82 * t42 - t44 * t96 - 2.0 * t70 * t71;
        v2rho2[ip] += tv2rho20;
        let t131 = t54 * t54;
        let t132 = 1.0 / t131;
        let t134 = t112 * t64;
        let t135 = t17 * t132 * t134;
        let t150 = t89 * t13;
        let t159 = -2.0 * t15 * t13 * t37 - t15 * t150 * t37 - 3.0 * t15 * t89 * t37 - 3.0 * t9 * t78;
        let t161 = t159 * t50 * t34;
        let t165 = t117 * t24;
        let t175 = t122 * t29;
        let t185 = -6.0 * t26 * t117 * t37 - 6.0 * t31 * t122 * t37 - 2.0 * t26 * t165 * t37 - 2.0 * t31 * t175 * t37 - 4.0 * t26 * t24 * t37 - 4.0 * t31 * t29 * t37 - 6.0 * t21 * t78;
        let t186 = t56 * t185;
        let t192 = t98 * t49;
        let t194 = 1.0 / t99 / t16;
        let t196 = t192 * t194 * t34;
        let t199 = t8 * t98;
        let t200 = t100 * t34;
        let t203 = t110 * t112;
        let t206 = 3.0 * t41 * t102 + 6.0 * t70 * t106 - 6.0 * t41 * t113 + 3.0 * t41 * t129 + 6.0 * t44 * t135 - t44 * t161 - 6.0 * t18 * t203 + t44 * t186 - 2.0 * t44 * t196 + 3.0 * t199 * t200 - 3.0 * t41 * t96 - 3.0 * t82 * t52 + 3.0 * t82 * t65;
        let t209 = t40 * t49;
        let t212 = t8 * t94;
        let t215 = t55 * t128;
        let t220 = 1.0 / t77 / rho[ip];
        let t223 = -6.0 * t3 * t220 - 3.0 * t78;
        let t224 = rho[ip] * t223;
        let t226 = t41 * t49;
        let t229 = t44 * t94;
        let t230 = t200 * t49;
        let t235 = t44 * t98;
        let t236 = t100 * t55;
        let t237 = t236 * t64;
        let t240 = t105 * t128;
        let t243 = t44 * t17;
        let t244 = t110 * t64;
        let t245 = t244 * t128;
        let t248 = t50 * t110;
        let t249 = t248 * t112;
        let t252 = t81 * t17;
        let t255 = 3.0 * t104 * t240 - 6.0 * t104 * t249 + 6.0 * t226 * t106 + 3.0 * t229 * t106 + 3.0 * t18 * t215 - 6.0 * t209 * t71 - 3.0 * t212 * t71 - t224 * t42 + 3.0 * t229 * t230 - 3.0 * t235 * t237 - 6.0 * t243 * t245 - 3.0 * t252 * t34 + 6.0 * t67 * t74;
        let tv3rho30 = t206 + t255;
        v3rho3[ip] += tv3rho30;
    }
}
