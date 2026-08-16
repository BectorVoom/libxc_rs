//! MGGA_K_GEA2 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea2.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_gea2_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
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
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t48 = 1.0 / t41 / rho0;
        let t52 = 1.0 + 5.0 / 648.0 * t38 * sigma0 * t43 + 5.0 / 54.0 * t38 * lapl0 * t48;
        let t56 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t18;
        let t60 = piecewise5(t16, t13, t12, t17, t58 * t9);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t25, t64 * t61);
        let t67 = t66 * t31;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t77 = 1.0 / t70 / rho1;
        let t81 = 1.0 + 5.0 / 648.0 * t38 * sigma2 * t72 + 5.0 / 54.0 * t38 * lapl1 * t77;
        let t85 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t67 * t81);
        let tzk0 = t56 + t85;
        zk[ip] += tzk0;
        let t86 = t8 * t8;
        let t87 = 1.0 / t86;
        let t88 = t18 * t87;
        let t90 = piecewise5(t12, 0.0, t16, 0.0, t9 - t88);
        let t93 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t90);
        let t94 = t93 * t31;
        let t98 = 1.0 / t30;
        let t99 = t29 * t98;
        let t102 = t7 * t99 * t52 / 10.0;
        let t105 = 1.0 / t41 / t39 / rho0;
        let t112 = -5.0 / 243.0 * t38 * sigma0 * t105 - 25.0 / 162.0 * t38 * lapl0 * t43;
        let t117 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t94 * t52 + t102 + 3.0 / 20.0 * t7 * t32 * t112);
        let t118 = t58 * t87;
        let t120 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t118);
        let t123 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t120);
        let t124 = t123 * t31;
        let t128 = t66 * t98;
        let t131 = t7 * t128 * t81 / 10.0;
        let t133 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t124 * t81 + t131);
        let tvrho0 = t56 + t85 + t8 * (t117 + t133);
        vrho[ip * 2] += tvrho0;
        let t137 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t88);
        let t140 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t137);
        let t141 = t140 * t31;
        let t146 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t141 * t52 + t102);
        let t148 = piecewise5(t16, 0.0, t12, 0.0, t9 - t118);
        let t151 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t148);
        let t152 = t151 * t31;
        let t158 = 1.0 / t70 / t68 / rho1;
        let t165 = -5.0 / 243.0 * t38 * sigma2 * t158 - 25.0 / 162.0 * t38 * lapl1 * t72;
        let t170 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t152 * t81 + t131 + 3.0 / 20.0 * t7 * t67 * t165);
        let tvrho1 = t56 + t85 + t8 * (t146 + t170);
        vrho[ip * 2 + 1] += tvrho1;
        let t173 = t7 * t29;
        let t174 = t31 * t33;
        let t175 = t37 * t43;
        let t176 = t174 * t175;
        let t177 = t173 * t176;
        let t179 = piecewise3(t2, 0.0, t177 / 864.0);
        let tvsigma0 = t8 * t179;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t180 = t7 * t66;
        let t181 = t37 * t72;
        let t182 = t174 * t181;
        let t183 = t180 * t182;
        let t185 = piecewise3(t57, 0.0, t183 / 864.0);
        let tvsigma2 = t8 * t185;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t186 = t37 * t48;
        let t187 = t174 * t186;
        let t190 = piecewise3(t2, 0.0, t173 * t187 / 72.0);
        let tvlapl0 = t8 * t190;
        vlapl[ip * 2] += tvlapl0;
        let t191 = t37 * t77;
        let t192 = t174 * t191;
        let t195 = piecewise3(t57, 0.0, t180 * t192 / 72.0);
        let tvlapl1 = t8 * t195;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
