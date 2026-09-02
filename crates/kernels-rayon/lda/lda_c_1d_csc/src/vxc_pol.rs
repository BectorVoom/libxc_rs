//! LDA_C_1D_CSC vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_csc_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = 1.0 / t1;
        let t3 = t2 / 2.0;
        let t4 = param_para_4;
        let t5 = t1 * t1;
        let t6 = 1.0 / t5;
        let t9 = t3 + t4 * t6 / 4.0;
        let t10 = param_para_7;
        let t14 = param_para_9;
        let t15 = rmath::pow(t3, t14);
        let t16 = param_para_8 * t15;
        let t17 = 1.0 + t10 * t2 / 2.0 + t16;
        let t18 = rmath::ln(t17);
        let t19 = t9 * t18;
        let t22 = param_para_1;
        let t25 = param_para_5;
        let t26 = rmath::pow(t3, t25);
        let t27 = param_para_2 * t26;
        let t30 = param_para_6;
        let t31 = rmath::pow(t3, t30);
        let t32 = param_para_3 * t31;
        let t34 = t22 * t2 + 2.0 * t27 + 2.0 * t32 + 2.0 * param_para_0;
        let t35 = 1.0 / t34;
        let t36 = t19 * t35;
        let t37 = param_ferro_4;
        let t40 = t3 + t37 * t6 / 4.0;
        let t41 = param_ferro_7;
        let t45 = param_ferro_9;
        let t46 = rmath::pow(t3, t45);
        let t47 = param_ferro_8 * t46;
        let t48 = 1.0 + t41 * t2 / 2.0 + t47;
        let t49 = rmath::ln(t48);
        let t50 = t40 * t49;
        let t53 = param_ferro_1;
        let t56 = param_ferro_5;
        let t57 = rmath::pow(t3, t56);
        let t58 = param_ferro_2 * t57;
        let t61 = param_ferro_6;
        let t62 = rmath::pow(t3, t61);
        let t63 = param_ferro_3 * t62;
        let t65 = t53 * t2 + 2.0 * t58 + 2.0 * t63 + 2.0 * param_ferro_0;
        let t66 = 1.0 / t65;
        let t68 = -t50 * t66 + t36;
        let t69 = rho0 - rho1;
        let t70 = t69 * t69;
        let t71 = t68 * t70;
        let t72 = t71 * t6;
        let tzk0 = -t36 + t72;
        zk[ip] += tzk0;
        let t74 = 1.0 / t5 / t1;
        let t77 = -t4 * t74 / 2.0 - t6 / 2.0;
        let t78 = t77 * t18;
        let t79 = t78 * t35;
        let t84 = -t10 * t6 / 2.0 - t16 * t14 * t2;
        let t85 = t9 * t84;
        let t86 = 1.0 / t17;
        let t87 = t86 * t35;
        let t88 = t85 * t87;
        let t89 = t34 * t34;
        let t90 = 1.0 / t89;
        let t98 = -2.0 * t27 * t25 * t2 - 2.0 * t32 * t30 * t2 - t22 * t6;
        let t99 = t90 * t98;
        let t100 = t19 * t99;
        let t103 = -t37 * t74 / 2.0 - t6 / 2.0;
        let t104 = t103 * t49;
        let t110 = -t41 * t6 / 2.0 - t47 * t45 * t2;
        let t111 = t40 * t110;
        let t112 = 1.0 / t48;
        let t113 = t112 * t66;
        let t115 = t65 * t65;
        let t116 = 1.0 / t115;
        let t124 = -2.0 * t58 * t56 * t2 - 2.0 * t63 * t61 * t2 - t53 * t6;
        let t125 = t116 * t124;
        let t127 = -t104 * t66 - t111 * t113 + t50 * t125 - t100 + t79 + t88;
        let t128 = t127 * t70;
        let t129 = t128 * t6;
        let t130 = t68 * t69;
        let t131 = t130 * t6;
        let t132 = 2.0 * t131;
        let t133 = t71 * t74;
        let t134 = 2.0 * t133;
        let tvrho0 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 + t132 - t134);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 - t132 - t134);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
