//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1318/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1318(t226: f64, t238: f64, t242: f64, t28834: f64, t10600: f64, t801: f64, t1329: f64, t8646: f64, t3329: f64, t2194: f64, t20691: f64, t20697: f64, t28794: f64, t28797: f64, t28800: f64, t28804: f64, t28808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28837 = t238 * t242 * t226 * t28834;
    let t28840 = t238 * t801 * t10600;
    let t28844 = t238 * t242 * t1329 * t8646;
    let t28846 = t3329 * t3329;
    let t28847 = t2194 * t28846;
    let t28849 = -0.14717333333333333333e1_f64 * t20691 + 0.27595e0_f64 * t20697 + 0.27595e0_f64 * t28794 - 0.33114e0_f64 * t28797 - 0.33114e0_f64 * t28800 + 0.248355e0_f64 * t28804 + 0.49671e0_f64 * t28808 + 0.248355e0_f64 * t28837 - 0.66228e0_f64 * t28840 + 0.49671e0_f64 * t28844 - 0.258925e1_f64 * t28847;
    (t28837, t28840, t28844, t28846, t28847, t28849)
}
