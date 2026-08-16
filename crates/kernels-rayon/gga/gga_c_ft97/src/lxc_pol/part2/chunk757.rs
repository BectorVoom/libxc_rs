//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 757/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk757(t1786: f64, t971: f64, t1905: f64, t463: f64, t1912: f64, t11878: f64, t11882: f64, t11883: f64, t11887: f64, t11897: f64, t11899: f64, t1901: f64, t446: f64, t8430: f64, t8471: f64, t8475: f64, t8477: f64, t8483: f64, t8485: f64, t8487: f64) -> f64 {
    let t11902 = t1786 * t971;
    let t11903 = t11902 * t1905;
    let t11906 = t463 * t971;
    let t11907 = t11906 * t1912;
    let t11910 = 2.0_f64 / 9.0_f64 * t446 * t11878 + t11882 - 4.0_f64 / 81.0_f64 * t11883 - t8430 / 9.0_f64 - t446 * t11887 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t8471 - 8.0_f64 / 27.0_f64 * t8475 + t8477 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t8483 - 8.0_f64 / 27.0_f64 * t8485 - 2.0_f64 / 9.0_f64 * t8487 - t11897 - 2.0_f64 / 3.0_f64 * t446 * t11899 + 2.0_f64 / 9.0_f64 * t1901 * t11903 + 2.0_f64 / 9.0_f64 * t1901 * t11907;
    t11910
}
