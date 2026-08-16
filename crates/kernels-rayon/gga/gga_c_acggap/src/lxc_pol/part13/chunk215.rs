//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 215/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk215(t688: f64, t690: f64, t286: f64, t104: f64, t96: f64) -> (f64, f64, f64) {
    let t691 = t688 * t690;
    let t692 = t286 * t691;
    let t693 = 0.17315859105681463759e2_f64 * t692;
    let t694 = t96 * t104;
    (t691, t693, t694)
}
