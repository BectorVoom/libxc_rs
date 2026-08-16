//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 448/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk448(t355: f64, t368: f64, t2095: f64, t1089: f64, t1095: f64, t2080: f64) -> (f64, f64, f64) {
    let t2096 = t368 * t355;
    let t2097 = t2095 * t2096;
    let t2098 = 0.7640625e-2_f64 * t2097;
    let t2100 = t1089 * t1095 * t2080;
    (t2096, t2098, t2100)
}
