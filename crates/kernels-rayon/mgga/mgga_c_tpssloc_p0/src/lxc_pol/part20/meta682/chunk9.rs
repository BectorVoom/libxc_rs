//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2583/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583(t51039: f64, t51051: f64, t43859: f64, t43861: f64, t43863: f64, t50968: f64, t50970: f64, t50972: f64, t50976: f64, t50978: f64, t50987: f64, t50990: f64, t51034: f64, t51037: f64, t51041: f64, t51043: f64, t51046: f64, t51049: f64, t51053: f64, t51056: f64) -> f64 {
    let t52339 = 10.0_f64 / 9.0_f64 * t51039;
    let t52343 = 5.0_f64 / 27.0_f64 * t51051;
    let t52345 = 40.0_f64 / 27.0_f64 * t43859 - 5.0_f64 / 9.0_f64 * t43861 - 10.0_f64 / 9.0_f64 * t43863 - 2.0_f64 / 9.0_f64 * t50968 - t50970 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t50972 + 14.0_f64 / 81.0_f64 * t50976 + 4.0_f64 / 27.0_f64 * t50978 - 2.0_f64 / 9.0_f64 * t50987 - 8.0_f64 / 9.0_f64 * t50990 - 2.0_f64 / 9.0_f64 * t51034 + t51037 - t52339 + 2.0_f64 / 3.0_f64 * t51041 + 2.0_f64 * t51043 + t51046 / 6.0_f64 + t51049 + t52343 + 4.0_f64 / 3.0_f64 * t51053 - t51056;
    t52345
}
