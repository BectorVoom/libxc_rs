//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 615/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk615(t1137: f64, t1805: f64, t1140: f64, t1809: f64, t1801: f64, t1797: f64, t1750: f64, t3431: f64, t174: f64, t1814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5842 = t1137 * t1805;
    let t5844 = t1140 * t1809;
    let t5846 = t1137 * t1801;
    let t5848 = t1140 * t1797;
    let t5850 = t3431 * t1750;
    let t5852 = t174 * t1814;
    (t5842, t5844, t5846, t5848, t5850, t5852)
}
