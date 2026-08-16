//! MGGA_X_PBE_GX vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_pbe_gx_vxc_unpol(
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
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t4 * t4;
        let t24 = M_CBRT4;
        let t26 = 8.0 / 27.0 * t21 * t22 * t24;
        let t27 = t21 * t21;
        let t28 = tau[ip] * t27;
        let t29 = t20 * t20;
        let t31 = 1.0 / t29 / rho[ip];
        let t33 = sigma[ip] * t27;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t29 / t34;
        let t37 = t33 * t36;
        let t39 = t28 * t31 - t37 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t39 * t40 * t45;
        let t48 = 0.827411e0 - 0.35753333333333333333e0 * t46;
        let t50 = 1.0 - 0.45341611111111111111e0 * t46;
        let t51 = 1.0 / t50;
        let t53 = 1.0 - t26;
        let t54 = t48 * t51 * t53;
        let t57 = t26 + 5.0 / 9.0 * t46 * t54;
        let t58 = 5.0 / 9.0 * t46;
        let t59 = 1.0 - t58;
        let t60 = Heaviside(t59);
        let t62 = 1.0 + t58;
        let t63 = 1.0 / t62;
        let t66 = 1.0 + 0.148e0 * t59 * t63;
        let t67 = -t59;
        let t68 = Heaviside(t67);
        let t70 = t57 * t60 + t66 * t68;
        let t73 = 1.0 + 0.1015549e-2 * t37;
        let t74 = 1.0 / t73;
        let t78 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t70 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t79 = 1.0 / t29;
        let t86 = t34 * rho[ip];
        let t88 = 1.0 / t29 / t86;
        let t91 = -5.0 / 3.0 * t28 * t36 + t33 * t88 / 3.0;
        let t92 = t91 * t40;
        let t93 = t92 * t45;
        let t96 = t40 * t40;
        let t97 = t39 * t96;
        let t99 = 1.0 / t43 / t42;
        let t100 = t97 * t99;
        let t102 = t91 * t51 * t53;
        let t105 = t50 * t50;
        let t106 = 1.0 / t105;
        let t107 = t48 * t106;
        let t108 = t53 * t91;
        let t109 = t107 * t108;
        let t112 = 5.0 / 9.0 * t93 * t54 - 0.19862962962962962963e0 * t100 * t102 + 0.25189783950617283951e0 * t100 * t109;
        let t114 = 0.0;
        let t115 = t57 * t114;
        let t118 = t45 * t63;
        let t121 = t62 * t62;
        let t122 = 1.0 / t121;
        let t123 = t59 * t122;
        let t126 = -0.82222222222222222222e-1 * t92 * t118 - 0.82222222222222222222e-1 * t123 * t93;
        let t128 = t66 * t114;
        let t131 = t112 * t60 - 5.0 / 9.0 * t115 * t93 + t126 * t68 + 5.0 / 9.0 * t128 * t93;
        let t136 = t4 * t18;
        let t138 = 1.0 / t20 / t86;
        let t139 = t136 * t138;
        let t140 = t73 * t73;
        let t141 = 1.0 / t140;
        let t142 = t70 * t141;
        let t143 = t142 * t33;
        let t147 = piecewise3(t3, 0.0, -t19 * t79 * t70 * t74 / 8.0 - 3.0 / 8.0 * t19 * t20 * t131 * t74 - 0.69340067265485227402e-3 * t139 * t143);
        let tvrho0 = 2.0 * rho[ip] * t147 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t150 = t27 * t36;
        let t153 = t51 * t53;
        let t154 = t45 * t48 * t153;
        let t155 = t150 * t40 * t154;
        let t158 = t100 * t150 * t153;
        let t160 = t99 * t48;
        let t161 = t97 * t160;
        let t162 = t106 * t53;
        let t164 = t161 * t162 * t150;
        let t166 = -5.0 / 72.0 * t155 + 0.24828703703703703703e-1 * t158 - 0.31487229938271604938e-1 * t164;
        let t168 = t115 * t27;
        let t170 = t36 * t40 * t45;
        let t171 = t168 * t170;
        let t173 = t40 * t45;
        let t174 = t173 * t63;
        let t175 = t150 * t174;
        let t177 = t123 * t27;
        let t178 = t177 * t170;
        let t180 = 0.10277777777777777778e-1 * t175 + 0.10277777777777777778e-1 * t178;
        let t182 = t128 * t27;
        let t183 = t182 * t170;
        let t185 = t166 * t60 + 5.0 / 72.0 * t171 + t180 * t68 - 5.0 / 72.0 * t183;
        let t192 = t136 / t20 / t34;
        let t193 = t142 * t27;
        let t197 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t185 * t74 + 0.26002525224556960275e-3 * t192 * t193);
        let tvsigma0 = 2.0 * rho[ip] * t197;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t199 = t27 * t31;
        let t209 = 5.0 / 9.0 * t199 * t40 * t154 - 0.19862962962962962963e0 * t100 * t199 * t153 + 0.25189783950617283951e0 * t161 * t162 * t199;
        let t212 = t31 * t40 * t45;
        let t219 = -0.82222222222222222222e-1 * t199 * t174 - 0.82222222222222222222e-1 * t177 * t212;
        let t223 = t209 * t60 - 5.0 / 9.0 * t168 * t212 + t219 * t68 + 5.0 / 9.0 * t182 * t212;
        let t228 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t223 * t74);
        let tvtau0 = 2.0 * rho[ip] * t228;
        vtau[ip] += tvtau0;
    }
}
