//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 801/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk801(t14422: f64, t285: f64, t25: f64, t4973: f64, t291: f64, t992: f64, t4958: f64, t984: f64, t4943: f64, t9938: f64, t991: f64, t2880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14423 = t285 * t14422;
    let t14425 = t25 * t4973;
    let t14427 = t285 * t14425 / 144.0_f64;
    let t14430 = t992 * t291;
    let t14439 = t984 * t4958 / 54.0_f64;
    let t14440 = t9938 * t4943;
    let t14442 = t991 * t14440 / 432.0_f64;
    let t14443 = t2880 * t291;
    (t14423, t14427, t14430, t14439, t14442, t14443)
}
