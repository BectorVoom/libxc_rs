//! LDA_X_REL lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_rel_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t8 = pow_1_3(zeta_threshold);
        let t10 = piecewise3(1.0 <= zeta_threshold, t8 * zeta_threshold, 1.0);
        let t11 = pow_1_3(rho[ip]);
        let t15 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t10 * t11);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t17 * t3;
        let t19 = 1.0 / M_PI;
        let t20 = pow_1_3(t19);
        let t21 = t20 * t20;
        let t22 = 1.0 / t21;
        let t23 = t11 * t11;
        let t27 = 1.0 + 3.8075239991386495e-05 * t18 * t22 * t23;
        let t28 = rmath::sqrt(t27);
        let t29 = t28 * t17;
        let t30 = t3 * t20;
        let t35 = t3 * t3;
        let t36 = t16 * t35;
        let t37 = 1.0 / t20;
        let t41 = rmath::ln(0.0035625477770544352 * t36 * t37 * t11 + rmath::sqrt(pow_2(0.0035625477770544352 * t36 * t37 * t11) + 1.0));
        let t42 = t41 * t16;
        let t43 = t35 * t21;
        let t44 = 1.0 / t23;
        let t48 = 10.396221848752237 * t29 * t30 / t11 - 972.7328585562606 * t42 * t43 * t44;
        let t49 = t48 * t48;
        let t51 = 1.0 - 1.5 * t49;
        let tzk0 = 2.0 * t15 * t51;
        zk[ip] += tzk0;
        let t56 = piecewise3(t2, 0.0, -t6 * t10 * t44 / 8.0);
        let t57 = rho[ip] * t56;
        let t60 = rho[ip] * t15;
        let t61 = 1.0 / t28;
        let t62 = t61 * t16;
        let t63 = t35 * t37;
        let t68 = 1.0 / t11 / rho[ip];
        let t69 = t30 * t68;
        let t72 = t61 * t17;
        let t76 = 1.0 / t23 / rho[ip];
        let t80 = 0.0011875159256848119 * t62 * t63 * t44 - 3.4654072829174125 * t29 * t69 - 3.4654072829174125 * t72 * t69 + 648.4885723708404 * t42 * t43 * t76;
        let t81 = t48 * t80;
        let tvrho0 = tzk0 + 2.0 * t57 * t51 - 6.0 * t60 * t81;
        vrho[ip] += tvrho0;
        let t86 = t15 * t48;
        let t92 = piecewise3(t2, 0.0, t6 * t10 * t76 / 12.0);
        let t93 = rho[ip] * t92;
        let t98 = t80 * t80;
        let t102 = 1.0 / t28 / t27;
        let t106 = t63 * t76;
        let t109 = rho[ip] * rho[ip];
        let t111 = 1.0 / t11 / t109;
        let t112 = t30 * t111;
        let t115 = t102 * t16;
        let t121 = 1.0 / t23 / t109;
        let t125 = -1.278422702025102e-06 * t102 / rho[ip] - 0.0011875159256848119 * t62 * t106 + 4.620543043889883 * t29 * t112 + 0.00039583864189493724 * t115 * t106 + 6.930814565834825 * t72 * t112 - 1080.814287284734 * t42 * t43 * t121;
        let t126 = t48 * t125;
        let tv2rho20 = 4.0 * t56 * t51 - 12.0 * t86 * t80 + 2.0 * t93 * t51 - 12.0 * t57 * t81 - 6.0 * t60 * t98 - 6.0 * t60 * t126;
        v2rho2[ip] += tv2rho20;
        let t131 = t56 * t48;
        let t141 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t10 * t121);
        let t142 = rho[ip] * t141;
        let t151 = t80 * t125;
        let t154 = t27 * t27;
        let t156 = 1.0 / t28 / t154;
        let t158 = t18 * t22;
        let t161 = 1.0 / t109;
        let t164 = t63 * t121;
        let t167 = t109 * rho[ip];
        let t170 = t30 / t11 / t167;
        let t180 = 1.0 / t23 / t167;
        let t184 = 4.867625119004254e-11 * t156 * t68 * t158 + 2.556845404050204e-06 * t102 * t161 + 0.0025069780653346027 * t62 * t164 - 10.781267102409728 * t29 * t170 - 1.278422702025102e-06 * t156 * t161 - 0.00145140835361477 * t115 * t164 - 20.022353190189495 * t72 * t170 + 2882.1714327592904 * t42 * t43 * t180;
        let t185 = t48 * t184;
        let tv3rho30 = 6.0 * t92 * t51 - 36.0 * t131 * t80 - 18.0 * t15 * t98 - 18.0 * t86 * t125 + 2.0 * t142 * t51 - 18.0 * t93 * t81 - 18.0 * t57 * t98 - 18.0 * t57 * t126 - 18.0 * t60 * t151 - 6.0 * t60 * t185;
        v3rho3[ip] += tv3rho30;
        let t205 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t10 * t180);
        let t219 = t125 * t125;
        let t227 = 1.0 / t28 / t154 / t27;
        let t237 = 1.0 / t167;
        let t240 = t63 * t180;
        let t243 = t109 * t109;
        let t246 = t30 / t11 / t243;
        let tv4rho40 = 8.0 * t141 * t51 - 72.0 * t92 * t48 * t80 - 72.0 * t56 * t98 - 72.0 * t131 * t125 - 72.0 * t15 * t80 * t125 - 24.0 * t86 * t184 + 2.0 * rho[ip] * t205 * t51 - 24.0 * t142 * t81 - 36.0 * t93 * t98 - 36.0 * t93 * t126 - 72.0 * t57 * t151 - 24.0 * t57 * t185 - 18.0 * t60 * t219 - 24.0 * t60 * t80 * t184 - 6.0 * t60 * t48 * (-2.7800399189128234e-14 * t227 * t76 * t36 / t20 / t19 - 1.6225417063347515e-10 * t156 * t111 * t158 - 7.81258317904229e-06 * t102 * t237 - 0.007916772837898746 * t62 * t240 + 35.93755700803243 * t29 * t246 + 8.112708531673758e-11 * t227 * t111 * t158 + 7.244395311475577e-06 * t156 * t237 + 0.006157489985032357 * t115 * t240 + 77.00905073149806 * t72 * t246 - 10567.9619201174 * t42 * t43 / t23 / t243);
        v4rho4[ip] += tv4rho40;
    }
}
