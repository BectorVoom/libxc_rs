//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2583/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2583(t11292: f64, t1687: f64, t11365: f64, t1694: f64, t3331: f64, t4794: f64, t14933: f64, t300: f64, t3401: f64, t11310: f64, t15823: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51680 = t1687 * t11292;
    let t51727 = t11365 * t1694;
    let t51730 = t4794 * t3331;
    let t51807 = t300 * t14933;
    let t51810 = t300 * t3401;
    let t51819 = t300 * t11310;
    let t51848 = t300 * t11365;
    let t51925 = t15823 * t225;
    (t51680, t51727, t51730, t51807, t51810, t51819, t51848, t51925)
}
