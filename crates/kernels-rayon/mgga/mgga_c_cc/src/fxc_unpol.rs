//! MGGA_C_CC fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cc_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(1.0 <= zeta_threshold, t35 * zeta_threshold, 1.0);
        let t40 = M_CBRT2;
        let t44 = (2.0 * t37 - 2.0) / (2.0 * t40 - 2.0);
        let t46 = 1.0 + 0.0278125 * t11;
        let t51 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let tzk0 = -t33 + t58;
        zk[ip] += tzk0;
        let t60 = 1.0 / t8 / rho[ip];
        let t61 = t7 * t60;
        let t63 = t5 * t61 * t31;
        let t65 = t27 * t27;
        let t66 = 1.0 / t65;
        let t67 = t13 * t66;
        let t69 = 1.0 / t14 * t2;
        let t70 = t4 * t7;
        let t71 = t70 * t60;
        let t72 = t69 * t71;
        let t74 = t5 * t61;
        let t76 = f64::sqrt(t11);
        let t77 = t76 * t2;
        let t78 = t77 * t71;
        let t83 = t21 * t6 / t22 / rho[ip];
        let t85 = -0.632975 * t72 - 0.29896666666666666 * t74 - 0.1023875 * t78 - 0.08215666666666667 * t83;
        let t86 = 1.0 / t30;
        let t87 = t85 * t86;
        let t88 = t67 * t87;
        let t90 = t44 * t2;
        let t93 = t90 * t70 * t60 * t55;
        let t95 = t44 * t46;
        let t96 = t51 * t51;
        let t97 = 1.0 / t96;
        let t102 = -0.8630833333333333 * t72 - 0.301925 * t74 - 0.05501625 * t78 - 0.082785 * t83;
        let t104 = 1.0 / t54;
        let t105 = t97 * t102 * t104;
        let t106 = t95 * t105;
        let tvrho0 = -t33 + t58 + rho[ip] * (0.0011073470983333333 * t63 + 1.0 * t88 - 0.00018311447306006544 * t93 - 0.5848223622634646 * t106);
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t114 = rho[ip] * rho[ip];
        let t116 = 1.0 / t8 / t114;
        let t117 = t7 * t116;
        let t119 = t5 * t117 * t31;
        let t121 = t5 * t7;
        let t122 = t60 * t66;
        let t124 = t121 * t122 * t87;
        let t126 = t65 * t27;
        let t127 = 1.0 / t126;
        let t128 = t13 * t127;
        let t129 = t85 * t85;
        let t130 = t129 * t86;
        let t131 = t128 * t130;
        let t135 = 1.0 / t14 / t11 * t19;
        let t136 = t20 * t6;
        let t138 = 1.0 / t22 / t114;
        let t139 = t136 * t138;
        let t140 = t135 * t139;
        let t142 = t70 * t116;
        let t143 = t69 * t142;
        let t145 = t5 * t117;
        let t147 = 1.0/f64::sqrt(t11);
        let t148 = t147 * t19;
        let t149 = t148 * t139;
        let t151 = t77 * t142;
        let t154 = t21 * t6 * t138;
        let t156 = -0.4219833333333333 * t140 + 0.8439666666666666 * t143 + 0.3986222222222222 * t145 + 0.06825833333333334 * t149 + 0.13651666666666668 * t151 + 0.1369277777777778 * t154;
        let t157 = t156 * t86;
        let t158 = t67 * t157;
        let t160 = t65 * t65;
        let t161 = 1.0 / t160;
        let t162 = t13 * t161;
        let t163 = t30 * t30;
        let t164 = 1.0 / t163;
        let t165 = t129 * t164;
        let t166 = t162 * t165;
        let t170 = t90 * t70 * t116 * t55;
        let t172 = t44 * t5;
        let t174 = t172 * t61 * t105;
        let t176 = t96 * t51;
        let t177 = 1.0 / t176;
        let t178 = t102 * t102;
        let t180 = t177 * t178 * t104;
        let t181 = t95 * t180;
        let t189 = -0.5753888888888888 * t140 + 1.1507777777777777 * t143 + 0.4025666666666667 * t145 + 0.0366775 * t149 + 0.073355 * t151 + 0.137975 * t154;
        let t191 = t97 * t189 * t104;
        let t192 = t95 * t191;
        let t194 = t96 * t96;
        let t195 = 1.0 / t194;
        let t196 = t195 * t178;
        let t197 = t54 * t54;
        let t198 = 1.0 / t197;
        let t199 = t196 * t198;
        let t200 = t95 * t199;
        let tv2rho20 = 0.0022146941966666666 * t63 + 2.0 * t88 - 0.0003662289461201309 * t93 - 1.1696447245269292 * t106 + rho[ip] * (-0.0014764627977777779 * t119 - 0.035616666666666665 * t124 - 2.0 * t131 + 1.0 * t158 + 16.081979498692537 * t166 + 0.00024415263074675396 * t170 + 0.01084358130030174 * t174 + 1.1696447245269292 * t181 - 0.5848223622634646 * t192 - 17.315859105681465 * t200);
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
