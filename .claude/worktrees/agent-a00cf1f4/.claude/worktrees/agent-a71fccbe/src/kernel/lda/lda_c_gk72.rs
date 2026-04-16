//! LDA C GK72 kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gk72.c`.

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT3, M_CBRT4};
use crate::math::piecewise::{piecewise5};
use crate::math::powers::{pow_1_3};

const M_PI_VAL: f64 = std::f64::consts::PI;

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        zk[ip] += tzk0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        let t52 = 1.0 / rho[ip];
        let t55 = 1.0 / t7 / rho[ip];
        let t56 = t6 * t55;
        let t66 = f64::powf(4.0, 1.0 / 6.0);
        let t67 = t66 * t48;
        let t68 = t4 * t55;
        let t76 = 1.0 / t30 / t2 / t52 / 48.0;
        let t77 = t66 * t76;
        let t81 = piecewise5(t12, -0.0103666666666667 * t52 - 0.00075 * t4 * t56 * t13 + 0.000666666666666667 * t4 * t56, t20, -0.00632666666666667 * t52, 0.0486666666666667 * t25 * t44 + 10.6 * t67 * t68 - 0.326666666666667 * t37 * t9 - 21.3333333333333 * t77 * t68);
        let tvrho0 = rho[ip] * t81 + tzk0;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        let t52 = 1.0 / rho[ip];
        let t55 = 1.0 / t7 / rho[ip];
        let t56 = t6 * t55;
        let t66 = f64::powf(4.0, 1.0 / 6.0);
        let t67 = t66 * t48;
        let t68 = t4 * t55;
        let t76 = 1.0 / t30 / t2 / t52 / 48.0;
        let t77 = t66 * t76;
        let t81 = piecewise5(t12, -0.0103666666666667 * t52 - 0.00075 * t4 * t56 * t13 + 0.000666666666666667 * t4 * t56, t20, -0.00632666666666667 * t52, 0.0486666666666667 * t25 * t44 + 10.6 * t67 * t68 - 0.326666666666667 * t37 * t9 - 21.3333333333333 * t77 * t68);
        let tvrho0 = rho[ip] * t81 + tzk0;
        let t84 = rho[ip] * rho[ip];
        let t85 = 1.0 / t84;
        let t88 = 1.0 / t7 / t84;
        let t89 = t6 * t88;
        let t99 = t5 / t38 / rho[ip];
        let t102 = t66 * t66;
        let t103 = t102 * t102;
        let t104 = t103 * t66;
        let t105 = t104 * t76;
        let t107 = 1.0 / t38 / t84;
        let t108 = t42 * t107;
        let t111 = t4 * t88;
        let t121 = 1.0 / t30 / t1 / t3 / t2 / t56 / 48.0;
        let t122 = t104 * t121;
        let t128 = piecewise5(t12, 0.0103666666666667 * t85 + 0.001 * t4 * t89 * t13 - 0.000638888888888889 * t4 * t89, t20, 0.00632666666666667 * t85, -0.0324444444444444 * t25 * t99 + 8.83333333333333 * t105 * t108 - 14.1333333333333 * t67 * t111 + 0.108888888888889 * t37 * t56 - 24.8888888888889 * t122 * t108 + 28.4444444444444 * t77 * t111);
        let tv2rho20 = rho[ip] * t128 + 2.0 * t81;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        let t52 = 1.0 / rho[ip];
        let t55 = 1.0 / t7 / rho[ip];
        let t56 = t6 * t55;
        let t66 = f64::powf(4.0, 1.0 / 6.0);
        let t67 = t66 * t48;
        let t68 = t4 * t55;
        let t76 = 1.0 / t30 / t2 / t52 / 48.0;
        let t77 = t66 * t76;
        let t81 = piecewise5(t12, -0.0103666666666667 * t52 - 0.00075 * t4 * t56 * t13 + 0.000666666666666667 * t4 * t56, t20, -0.00632666666666667 * t52, 0.0486666666666667 * t25 * t44 + 10.6 * t67 * t68 - 0.326666666666667 * t37 * t9 - 21.3333333333333 * t77 * t68);
        let tvrho0 = rho[ip] * t81 + tzk0;
        let t84 = rho[ip] * rho[ip];
        let t85 = 1.0 / t84;
        let t88 = 1.0 / t7 / t84;
        let t89 = t6 * t88;
        let t99 = t5 / t38 / rho[ip];
        let t102 = t66 * t66;
        let t103 = t102 * t102;
        let t104 = t103 * t66;
        let t105 = t104 * t76;
        let t107 = 1.0 / t38 / t84;
        let t108 = t42 * t107;
        let t111 = t4 * t88;
        let t121 = 1.0 / t30 / t1 / t3 / t2 / t56 / 48.0;
        let t122 = t104 * t121;
        let t128 = piecewise5(t12, 0.0103666666666667 * t85 + 0.001 * t4 * t89 * t13 - 0.000638888888888889 * t4 * t89, t20, 0.00632666666666667 * t85, -0.0324444444444444 * t25 * t99 + 8.83333333333333 * t105 * t108 - 14.1333333333333 * t67 * t111 + 0.108888888888889 * t37 * t56 - 24.8888888888889 * t122 * t108 + 28.4444444444444 * t77 * t111);
        let tv2rho20 = rho[ip] * t128 + 2.0 * t81;
        let t131 = t84 * rho[ip];
        let t132 = 1.0 / t131;
        let t135 = 1.0 / t7 / t131;
        let t136 = t6 * t135;
        let t147 = t29 * t121;
        let t148 = t84 * t84;
        let t149 = 1.0 / t148;
        let t150 = t2 * t149;
        let t154 = 1.0 / t38 / t131;
        let t155 = t42 * t154;
        let t158 = t4 * t135;
        let t168 = 1.0 / t30 / t23 / t35 / t2 / t99 / 192.0;
        let t169 = t29 * t168;
        let t177 = piecewise5(t12, -0.0207333333333333 * t132 - 0.00233333333333333 * t4 * t136 * t13 + 0.00115740740740741 * t4 * t136, t20, -0.0126533333333333 * t132, 0.0540740740740741 * t25 * t5 * t107 + 123.666666666667 * t147 * t150 - 35.3333333333333 * t105 * t155 + 32.9777777777778 * t67 * t158 - 0.145185185185185 * t37 * t89 - 448.0 * t169 * t150 + 99.5555555555556 * t122 * t155 - 66.3703703703704 * t77 * t158);
        let tv3rho30 = rho[ip] * t177 + 3.0 * t128;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
        v3rho3[ip] += tv3rho30;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        let t52 = 1.0 / rho[ip];
        let t55 = 1.0 / t7 / rho[ip];
        let t56 = t6 * t55;
        let t66 = f64::powf(4.0, 1.0 / 6.0);
        let t67 = t66 * t48;
        let t68 = t4 * t55;
        let t76 = 1.0 / t30 / t2 / t52 / 48.0;
        let t77 = t66 * t76;
        let t81 = piecewise5(t12, -0.0103666666666667 * t52 - 0.00075 * t4 * t56 * t13 + 0.000666666666666667 * t4 * t56, t20, -0.00632666666666667 * t52, 0.0486666666666667 * t25 * t44 + 10.6 * t67 * t68 - 0.326666666666667 * t37 * t9 - 21.3333333333333 * t77 * t68);
        let tvrho0 = rho[ip] * t81 + tzk0;
        let t84 = rho[ip] * rho[ip];
        let t85 = 1.0 / t84;
        let t88 = 1.0 / t7 / t84;
        let t89 = t6 * t88;
        let t99 = t5 / t38 / rho[ip];
        let t102 = t66 * t66;
        let t103 = t102 * t102;
        let t104 = t103 * t66;
        let t105 = t104 * t76;
        let t107 = 1.0 / t38 / t84;
        let t108 = t42 * t107;
        let t111 = t4 * t88;
        let t121 = 1.0 / t30 / t1 / t3 / t2 / t56 / 48.0;
        let t122 = t104 * t121;
        let t128 = piecewise5(t12, 0.0103666666666667 * t85 + 0.001 * t4 * t89 * t13 - 0.000638888888888889 * t4 * t89, t20, 0.00632666666666667 * t85, -0.0324444444444444 * t25 * t99 + 8.83333333333333 * t105 * t108 - 14.1333333333333 * t67 * t111 + 0.108888888888889 * t37 * t56 - 24.8888888888889 * t122 * t108 + 28.4444444444444 * t77 * t111);
        let tv2rho20 = rho[ip] * t128 + 2.0 * t81;
        let t131 = t84 * rho[ip];
        let t132 = 1.0 / t131;
        let t135 = 1.0 / t7 / t131;
        let t136 = t6 * t135;
        let t147 = t29 * t121;
        let t148 = t84 * t84;
        let t149 = 1.0 / t148;
        let t150 = t2 * t149;
        let t154 = 1.0 / t38 / t131;
        let t155 = t42 * t154;
        let t158 = t4 * t135;
        let t168 = 1.0 / t30 / t23 / t35 / t2 / t99 / 192.0;
        let t169 = t29 * t168;
        let t177 = piecewise5(t12, -0.0207333333333333 * t132 - 0.00233333333333333 * t4 * t136 * t13 + 0.00115740740740741 * t4 * t136, t20, -0.0126533333333333 * t132, 0.0540740740740741 * t25 * t5 * t107 + 123.666666666667 * t147 * t150 - 35.3333333333333 * t105 * t155 + 32.9777777777778 * t67 * t158 - 0.145185185185185 * t37 * t89 - 448.0 * t169 * t150 + 99.5555555555556 * t122 * t155 - 66.3703703703704 * t77 * t158);
        let tv3rho30 = rho[ip] * t177 + 3.0 * t128;
        let t182 = 1.0 / t7 / t148;
        let t183 = t6 * t182;
        let t196 = t148 * rho[ip];
        let t200 = 1.0 / t7 / t196 * t1 * t3;
        let t204 = t2 / t196;
        let t209 = t42 / t38 / t148;
        let t212 = t4 * t182;
        let t217 = M_PI_VAL * M_PI_VAL;
        let t234 = piecewise5(t12, 0.0622 * t149 + 0.00777777777777778 * t4 * t183 * t13 - 0.00308024691358025 * t4 * t183, t20, 0.03796 * t149, -0.144197530864198 * t25 * t5 * t154 + 742.0 * t66 * t168 * t2 * t200 - 989.333333333333 * t147 * t204 + 157.037037037037 * t105 * t209 - 109.925925925926 * t67 * t212 + 0.338765432098765 * t37 * t136 - 1.42592592592593 * t66 / t30 * t217 / t85 * t2 * t200 + 3584.0 * t169 * t204 - 442.469135802469 * t122 * t209 + 221.234567901235 * t77 * t212);
        let tv4rho40 = rho[ip] * t234 + 4.0 * t177;
        zk[ip] += tzk0;
        vrho[ip] += tvrho0;
        v2rho2[ip] += tv2rho20;
        v3rho3[ip] += tv3rho30;
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        zk[ip] += tzk0;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.0103666666666667 * t53 - 0.00075 * t4 * t57 * t14 + 0.000666666666666667 * t4 * t57, t21, -0.00632666666666667 * t53, 0.0486666666666667 * t26 * t45 + 10.6 * t68 * t69 - 0.326666666666667 * t38 * t10 - 21.3333333333333 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        let tvrho1 = tvrho0;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.0103666666666667 * t53 - 0.00075 * t4 * t57 * t14 + 0.000666666666666667 * t4 * t57, t21, -0.00632666666666667 * t53, 0.0486666666666667 * t26 * t45 + 10.6 * t68 * t69 - 0.326666666666667 * t38 * t10 - 21.3333333333333 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        let tvrho1 = tvrho0;
        let t85 = t7 * t7;
        let t86 = 1.0 / t85;
        let t89 = 1.0 / t8 / t85;
        let t90 = t6 * t89;
        let t100 = t5 / t39 / t7;
        let t103 = t67 * t67;
        let t104 = t103 * t103;
        let t105 = t104 * t67;
        let t106 = t105 * t77;
        let t108 = 1.0 / t39 / t85;
        let t109 = t43 * t108;
        let t112 = t4 * t89;
        let t122 = 1.0 / t31 / t1 / t3 / t2 / t57 / 48.0;
        let t123 = t105 * t122;
        let t129 = piecewise5(t13, 0.0103666666666667 * t86 + 0.001 * t4 * t90 * t14 - 0.000638888888888889 * t4 * t90, t21, 0.00632666666666667 * t86, -0.0324444444444444 * t26 * t100 + 8.83333333333333 * t106 * t109 - 14.1333333333333 * t68 * t112 + 0.108888888888889 * t38 * t57 - 24.8888888888889 * t123 * t109 + 28.4444444444444 * t78 * t112);
        let tv2rho20 = t7 * t129 + 2.0 * t82;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.0103666666666667 * t53 - 0.00075 * t4 * t57 * t14 + 0.000666666666666667 * t4 * t57, t21, -0.00632666666666667 * t53, 0.0486666666666667 * t26 * t45 + 10.6 * t68 * t69 - 0.326666666666667 * t38 * t10 - 21.3333333333333 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        let tvrho1 = tvrho0;
        let t85 = t7 * t7;
        let t86 = 1.0 / t85;
        let t89 = 1.0 / t8 / t85;
        let t90 = t6 * t89;
        let t100 = t5 / t39 / t7;
        let t103 = t67 * t67;
        let t104 = t103 * t103;
        let t105 = t104 * t67;
        let t106 = t105 * t77;
        let t108 = 1.0 / t39 / t85;
        let t109 = t43 * t108;
        let t112 = t4 * t89;
        let t122 = 1.0 / t31 / t1 / t3 / t2 / t57 / 48.0;
        let t123 = t105 * t122;
        let t129 = piecewise5(t13, 0.0103666666666667 * t86 + 0.001 * t4 * t90 * t14 - 0.000638888888888889 * t4 * t90, t21, 0.00632666666666667 * t86, -0.0324444444444444 * t26 * t100 + 8.83333333333333 * t106 * t109 - 14.1333333333333 * t68 * t112 + 0.108888888888889 * t38 * t57 - 24.8888888888889 * t123 * t109 + 28.4444444444444 * t78 * t112);
        let tv2rho20 = t7 * t129 + 2.0 * t82;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        let t132 = t85 * t7;
        let t133 = 1.0 / t132;
        let t136 = 1.0 / t8 / t132;
        let t137 = t6 * t136;
        let t148 = t30 * t122;
        let t149 = t85 * t85;
        let t150 = 1.0 / t149;
        let t151 = t2 * t150;
        let t155 = 1.0 / t39 / t132;
        let t156 = t43 * t155;
        let t159 = t4 * t136;
        let t169 = 1.0 / t31 / t24 / t36 / t2 / t100 / 192.0;
        let t170 = t30 * t169;
        let t178 = piecewise5(t13, -0.0207333333333333 * t133 - 0.00233333333333333 * t4 * t137 * t14 + 0.00115740740740741 * t4 * t137, t21, -0.0126533333333333 * t133, 0.0540740740740741 * t26 * t5 * t108 + 123.666666666667 * t148 * t151 - 35.3333333333333 * t106 * t156 + 32.9777777777778 * t68 * t159 - 0.145185185185185 * t38 * t90 - 448.0 * t170 * t151 + 99.5555555555556 * t123 * t156 - 66.3703703703704 * t78 * t159);
        let tv3rho30 = t7 * t178 + 3.0 * t129;
        let tv3rho31 = tv3rho30;
        let tv3rho32 = tv3rho31;
        let tv3rho33 = tv3rho32;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        v3rho3[ip * 4 + 0] += tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    #[allow(unused_variables)] dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI_VAL;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.0103666666666667 * t53 - 0.00075 * t4 * t57 * t14 + 0.000666666666666667 * t4 * t57, t21, -0.00632666666666667 * t53, 0.0486666666666667 * t26 * t45 + 10.6 * t68 * t69 - 0.326666666666667 * t38 * t10 - 21.3333333333333 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        let tvrho1 = tvrho0;
        let t85 = t7 * t7;
        let t86 = 1.0 / t85;
        let t89 = 1.0 / t8 / t85;
        let t90 = t6 * t89;
        let t100 = t5 / t39 / t7;
        let t103 = t67 * t67;
        let t104 = t103 * t103;
        let t105 = t104 * t67;
        let t106 = t105 * t77;
        let t108 = 1.0 / t39 / t85;
        let t109 = t43 * t108;
        let t112 = t4 * t89;
        let t122 = 1.0 / t31 / t1 / t3 / t2 / t57 / 48.0;
        let t123 = t105 * t122;
        let t129 = piecewise5(t13, 0.0103666666666667 * t86 + 0.001 * t4 * t90 * t14 - 0.000638888888888889 * t4 * t90, t21, 0.00632666666666667 * t86, -0.0324444444444444 * t26 * t100 + 8.83333333333333 * t106 * t109 - 14.1333333333333 * t68 * t112 + 0.108888888888889 * t38 * t57 - 24.8888888888889 * t123 * t109 + 28.4444444444444 * t78 * t112);
        let tv2rho20 = t7 * t129 + 2.0 * t82;
        let tv2rho21 = tv2rho20;
        let tv2rho22 = tv2rho21;
        let t132 = t85 * t7;
        let t133 = 1.0 / t132;
        let t136 = 1.0 / t8 / t132;
        let t137 = t6 * t136;
        let t148 = t30 * t122;
        let t149 = t85 * t85;
        let t150 = 1.0 / t149;
        let t151 = t2 * t150;
        let t155 = 1.0 / t39 / t132;
        let t156 = t43 * t155;
        let t159 = t4 * t136;
        let t169 = 1.0 / t31 / t24 / t36 / t2 / t100 / 192.0;
        let t170 = t30 * t169;
        let t178 = piecewise5(t13, -0.0207333333333333 * t133 - 0.00233333333333333 * t4 * t137 * t14 + 0.00115740740740741 * t4 * t137, t21, -0.0126533333333333 * t133, 0.0540740740740741 * t26 * t5 * t108 + 123.666666666667 * t148 * t151 - 35.3333333333333 * t106 * t156 + 32.9777777777778 * t68 * t159 - 0.145185185185185 * t38 * t90 - 448.0 * t170 * t151 + 99.5555555555556 * t123 * t156 - 66.3703703703704 * t78 * t159);
        let tv3rho30 = t7 * t178 + 3.0 * t129;
        let tv3rho31 = tv3rho30;
        let tv3rho32 = tv3rho31;
        let tv3rho33 = tv3rho32;
        let t183 = 1.0 / t8 / t149;
        let t184 = t6 * t183;
        let t197 = t149 * t7;
        let t201 = 1.0 / t8 / t197 * t1 * t3;
        let t205 = t2 / t197;
        let t210 = t43 / t39 / t149;
        let t213 = t4 * t183;
        let t218 = M_PI_VAL * M_PI_VAL;
        let t235 = piecewise5(t13, 0.0622 * t150 + 0.00777777777777778 * t4 * t184 * t14 - 0.00308024691358025 * t4 * t184, t21, 0.03796 * t150, -0.144197530864198 * t26 * t5 * t155 + 742.0 * t67 * t169 * t2 * t201 - 989.333333333333 * t148 * t205 + 157.037037037037 * t106 * t210 - 109.925925925926 * t68 * t213 + 0.338765432098765 * t38 * t137 - 1.42592592592593 * t67 / t31 * t218 / t86 * t2 * t201 + 3584.0 * t170 * t205 - 442.469135802469 * t123 * t210 + 221.234567901235 * t78 * t213);
        let tv4rho40 = t7 * t235 + 4.0 * t178;
        let tv4rho41 = tv4rho40;
        let tv4rho42 = tv4rho41;
        let tv4rho43 = tv4rho42;
        let tv4rho44 = tv4rho43;
        zk[ip] += tzk0;
        vrho[ip * 2 + 0] += tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        v2rho2[ip * 3 + 0] += tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        v3rho3[ip * 4 + 0] += tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        v4rho4[ip * 5 + 0] += tv4rho40;
        v4rho4[ip * 5 + 1] += tv4rho41;
        v4rho4[ip * 5 + 2] += tv4rho42;
        v4rho4[ip * 5 + 3] += tv4rho43;
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
