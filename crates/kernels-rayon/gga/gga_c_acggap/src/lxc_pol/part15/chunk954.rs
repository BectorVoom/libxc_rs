//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 954/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk954(t2016: f64, t8622: f64, t515: f64, t7852: f64, t2294: f64, t7630: f64, t31253: f64, t527: f64, t2299: f64, t7610: f64, t30037: f64, t2310: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33859 = t2016 * t8622;
    let t33861 = t7852 * t515;
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33872 = 0.68598428988911579156e-2_f64 * t30037;
    let t33874 = t7610 * t2310;
    (t33859, t33861, t33865, t33867, t33869, t33872, t33874)
}
