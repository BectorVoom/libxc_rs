//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1094/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1094(t7839: f64, t9593: f64, t1165: f64, t2068: f64, t38837: f64, t8600: f64, t1089: f64, t2090: f64, t27338: f64, t598: f64, t30364: f64, t6184: f64) -> (f64, f64, f64, f64) {
    let t38899 = t7839 * t9593;
    let t38903 = t2068 * t1165 * t8600 * t38837;
    let t38909 = t598 * t1089 * t27338 * t2090;
    let t38912 = t30364 * t6184;
    (t38899, t38903, t38909, t38912)
}
