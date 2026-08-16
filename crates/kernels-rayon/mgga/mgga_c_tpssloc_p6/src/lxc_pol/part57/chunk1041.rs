//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1041/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1041(t128639: f64, t128663: f64, t128701: f64, t128726: f64, t128761: f64, t128789: f64, t128818: f64, t128902: f64, t1390: f64, t1983: f64, t533: f64, t28821: f64, t8641: f64) -> (f64, f64) {
    let t128908 = t1983 * t533 * (t128639 + t128663 + t128701 + t128726 + t128761 + t128789 + t128818 + t128902) * t1390;
    let t128909 = t28821 * t8641;
    (t128908, t128909)
}
