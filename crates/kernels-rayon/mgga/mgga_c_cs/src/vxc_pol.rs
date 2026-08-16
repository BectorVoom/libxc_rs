//! MGGA_C_CS vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cs_vxc_pol(
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
        let t2 = rho0 - rho1;
        let t3 = t2 * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = 1.0 / t5;
        let t8 = -t3 * t6 + 1.0;
        let t9 = pow_1_3(t4);
        let t10 = 1.0 / t9;
        let t12 = 1.0 + 0.34899999999999999998e0 * t10;
        let t13 = 1.0 / t12;
        let t14 = t8 * t13;
        let t16 = f64::exp(-0.2533e0 * t10);
        let t17 = 1.0 / t4;
        let t18 = t2 * t17;
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = zeta_threshold * zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t25 = t19 * t19;
        let t26 = pow_1_3(t19);
        let t27 = t26 * t26;
        let t29 = piecewise3(t20, t24, t27 * t25);
        let t30 = M_CBRT2;
        let t31 = t29 * t30;
        let t32 = pow_1_3(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / rho0;
        let t37 = lapl0 * t35;
        let t39 = tau0 * t35 - t37 / 8.0;
        let t41 = 1.0 - t18;
        let t42 = t41 <= zeta_threshold;
        let t43 = t41 * t41;
        let t44 = pow_1_3(t41);
        let t45 = t44 * t44;
        let t47 = piecewise3(t42, t24, t45 * t43);
        let t48 = t47 * t30;
        let t49 = pow_1_3(rho1);
        let t50 = t49 * t49;
        let t52 = 1.0 / t50 / rho1;
        let t54 = lapl1 * t52;
        let t56 = tau1 * t52 - t54 / 8.0;
        let t59 = sigma0 + 2.0 * sigma1 + sigma2;
        let t60 = t9 * t9;
        let t62 = 1.0 / t60 / t5;
        let t64 = t19 / 2.0;
        let t65 = pow_1_3(t64);
        let t66 = t65 * t65;
        let t67 = t66 * t64;
        let t69 = t41 / 2.0;
        let t70 = pow_1_3(t69);
        let t71 = t70 * t70;
        let t72 = t71 * t69;
        let t75 = t31 * t39 / 8.0 + t37 * t67 / 8.0 + t48 * t56 / 8.0 + t54 * t72 / 8.0 - t59 * t62 / 8.0;
        let t78 = 1.0 + 0.264e0 * t16 * t75;
        let tzk0 = -0.4918e-1 * t14 * t78;
        zk[ip] += tzk0;
        let t81 = t2 * t6;
        let t82 = t5 * t4;
        let t83 = 1.0 / t82;
        let t84 = t3 * t83;
        let t86 = -2.0 * t81 + 2.0 * t84;
        let t87 = t4 * t86;
        let t88 = t13 * t78;
        let t91 = t10 * t8;
        let t92 = t12 * t12;
        let t93 = 1.0 / t92;
        let t94 = t93 * t78;
        let t96 = 0.57212733333333333332e-2 * t91 * t94;
        let t97 = t4 * t8;
        let t99 = 1.0 / t9 / t4;
        let t100 = t99 * t16;
        let t102 = 0.222904e-1 * t100 * t75;
        let t103 = t27 * t19;
        let t104 = t17 - t81;
        let t107 = piecewise3(t20, 0.0, 8.0 / 3.0 * t103 * t104);
        let t108 = t107 * t30;
        let t111 = rho0 * rho0;
        let t113 = 1.0 / t33 / t111;
        let t116 = lapl0 * t113;
        let t118 = -5.0 / 3.0 * tau0 * t113 + 5.0 / 24.0 * t116;
        let t121 = t45 * t41;
        let t122 = -t104;
        let t125 = piecewise3(t42, 0.0, 8.0 / 3.0 * t121 * t122);
        let t126 = t125 * t30;
        let t130 = 1.0 / t60 / t82;
        let t132 = t59 * t130 / 3.0;
        let t135 = t104 / 2.0;
        let t136 = t66 * t135;
        let t139 = -t135;
        let t140 = t71 * t139;
        let t143 = t108 * t39 / 8.0 + t31 * t118 / 8.0 + t126 * t56 / 8.0 + t132 - 5.0 / 24.0 * t116 * t67 + 5.0 / 24.0 * t37 * t136 + 5.0 / 24.0 * t54 * t140;
        let t146 = t102 + 0.264e0 * t16 * t143;
        let t147 = t13 * t146;
        let tvrho0 = tzk0 - 0.4918e-1 * t87 * t88 - t96 - 0.4918e-1 * t97 * t147;
        vrho[ip * 2] += tvrho0;
        let t151 = 2.0 * t81 + 2.0 * t84;
        let t152 = t4 * t151;
        let t155 = -t17 - t81;
        let t158 = piecewise3(t20, 0.0, 8.0 / 3.0 * t103 * t155);
        let t159 = t158 * t30;
        let t162 = -t155;
        let t165 = piecewise3(t42, 0.0, 8.0 / 3.0 * t121 * t162);
        let t166 = t165 * t30;
        let t169 = rho1 * rho1;
        let t171 = 1.0 / t50 / t169;
        let t174 = lapl1 * t171;
        let t176 = -5.0 / 3.0 * tau1 * t171 + 5.0 / 24.0 * t174;
        let t179 = t155 / 2.0;
        let t180 = t66 * t179;
        let t185 = -t179;
        let t186 = t71 * t185;
        let t189 = t159 * t39 / 8.0 + t166 * t56 / 8.0 + t48 * t176 / 8.0 + t132 + 5.0 / 24.0 * t37 * t180 - 5.0 / 24.0 * t174 * t72 + 5.0 / 24.0 * t54 * t186;
        let t192 = t102 + 0.264e0 * t16 * t189;
        let t193 = t13 * t192;
        let tvrho1 = tzk0 - 0.4918e-1 * t152 * t88 - t96 - 0.4918e-1 * t97 * t193;
        vrho[ip * 2 + 1] += tvrho1;
        let t197 = 1.0 / t60 / t4;
        let t198 = t197 * t8;
        let t199 = t13 * t16;
        let t200 = t198 * t199;
        let tvsigma0 = 0.162294e-2 * t200;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.324588e-2 * t200;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t201 = t31 * t35;
        let t205 = -t201 / 64.0 + t35 * t67 / 8.0;
        let t206 = t199 * t205;
        let tvlapl0 = -0.1298352e-1 * t97 * t206;
        vlapl[ip * 2] += tvlapl0;
        let t209 = t48 * t52;
        let t213 = -t209 / 64.0 + t52 * t72 / 8.0;
        let t214 = t199 * t213;
        let tvlapl1 = -0.1298352e-1 * t97 * t214;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t217 = t97 * t13;
        let t218 = t16 * t29;
        let t219 = t30 * t35;
        let t220 = t218 * t219;
        let tvtau0 = -0.162294e-2 * t217 * t220;
        vtau[ip * 2] += tvtau0;
        let t223 = t16 * t47;
        let t224 = t30 * t52;
        let t225 = t223 * t224;
        let tvtau1 = -0.162294e-2 * t217 * t225;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
