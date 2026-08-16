//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 594/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk594(t5296: f64, t5297: f64, t1042: f64, t3362: f64, t3617: f64, t4181: f64, t1012: f64, t1224: f64, t5052: f64, t3698: f64, t5047: f64, t482: f64, t5245: f64) -> (f64, f64, f64, f64, f64) {
    let t5298 = t5296 * t5297;
    let t5299 = t1042 * t5298;
    let t5302 = t3617 * t3362;
    let t5303 = t5302 * t4181;
    let t5304 = t1042 * t5303;
    let t5308 = t1012 * t1224;
    let t5309 = t5308 * t5052;
    let t5312 = t1012 * t3698;
    let t5313 = t5312 * t5047;
    let t5318 = t482 * t5245;
    (t5299, t5304, t5309, t5313, t5318)
}
