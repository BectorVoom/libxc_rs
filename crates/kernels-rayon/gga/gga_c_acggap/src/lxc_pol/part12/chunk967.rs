//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 967/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk967(t29984: f64, t315: f64, t30028: f64, t2130: f64, t3874: f64, t615: f64, t7930: f64, t7911: f64, t862: f64, t13483: f64, t614: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32063 = t315 * t29984;
    let t32092 = t315 * t30028;
    let t32123 = t2130 * t3874;
    let t32124 = t615 * t32123;
    let t32130 = t315 * t7930;
    let t32142 = t862 * t7911;
    let t32146 = t614 * t13483 * t2130;
    let t32165 = t851 * t2130;
    (t32063, t32092, t32123, t32124, t32130, t32142, t32146, t32165)
}
