//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1030/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1030(t22633: f64, t22635: f64, t31549: f64, t6347: f64, t22685: f64, t28191: f64, t31611: f64, t1985: f64, t8621: f64, t97511: f64, t102922: f64, t122152: f64, t127203: f64, t127210: f64, t1375: f64, t1842: f64, t2016: f64, t2091: f64, t2092: f64, t26224: f64, t27009: f64, t28186: f64, t28220: f64, t28223: f64, t28224: f64, t29361: f64, t33293: f64, t33301: f64, t33316: f64, t3887: f64, t5321: f64, t6958: f64, t7194: f64, t7729: f64, t93319: f64, t97756: f64) -> f64 {
    let t128671 = t22633 * t22635 * t31549 * t6347;
    let t128691 = t22685 * t31611 * t28191;
    let t128694 = t1985 * t97511 * t8621;
    let t128701 = 4.0_f64 * t5321 * t33316 + 4.0_f64 * t7194 * t28220 - t127203 + 0.16449340668482264365e-1_f64 * t128671 - 0.38381794893125283518e-1_f64 * t122152 + 4.0_f64 * t1375 * t3887 * t33293 * t1842 + t127210 + 4.0_f64 * t27009 * t7729 + 2.0_f64 * t1375 * t3887 * t2091 * t28186 + 4.0_f64 * t5321 * t33301 - 2.0_f64 * t97756 * t2092 - 6.0_f64 * t7194 * t28224 + 0.49348022005446793095e-1_f64 * t128691 - 0.82246703342411321825e-2_f64 * t128694 - t6958 * t29361 - t102922 * t2016 + 24.0_f64 * t26224 * t93319 * t28223;
    t128701
}
