//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 616/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk616(t2937: f64, t406: f64, t1165: f64, t5852: f64, t1533: f64, t174: f64, t1838: f64) -> (f64, f64, f64, f64) {
    let t5853 = t2937 * t406;
    let t5855 = t1165 * t5852 * t5853;
    let t5859 = t1165 * t5852 * t1533;
    let t5862 = t174 * t1838;
    (t5853, t5855, t5859, t5862)
}
