//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 216/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk216(t132: f64, t250: f64, t249: f64, t67: f64, t62: f64, t256: f64) -> (f64, f64, f64, f64, f64) {
    let t722 = t132 * t250;
    let t726 = t249 * t67;
    let t727 = 1.0_f64 / t726;
    let t728 = t62 * t727;
    let t729 = t256 * t256;
    (t722, t726, t727, t728, t729)
}
