//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 917/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk917(t9947: f64, t9957: f64, t871: f64, t3801: f64, t881: f64, t6090: f64, t6127: f64, t7955: f64, t8038: f64, t9782: f64, t9797: f64, t378: f64) -> (f64, f64, f64, f64, f64) {
    let t9958 = t9947 + t9957;
    let t9959 = t9958 * t871;
    let t9964 = t3801 * t881;
    let t9973 = -t6127 + 0.12361111111111111111e-1_f64 * t6090 + 0.24722222222222222223e-1_f64 * t7955 - t8038 - 0.92708333333333333333e-2_f64 * t9782 + 0.278125e-1_f64 * t9797;
    let t9974 = t9973 * t378;
    (t9958, t9959, t9964, t9973, t9974)
}
