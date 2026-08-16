//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 856/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk856(t1345: f64, t322: f64, t1662: f64, t301: f64, t467: f64, t495: f64, t811: f64, t7884: f64, t7911: f64, t7930: f64, t862: f64, t309: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23745 = t1345 * t322;
    let t24589 = t301 * t1662;
    let t24605 = t1662 * t467;
    let t24623 = t495 * t811;
    let t29976 = t7884 * t7911;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    (t23745, t24589, t24605, t24623, t29976, t29979, t29980)
}
