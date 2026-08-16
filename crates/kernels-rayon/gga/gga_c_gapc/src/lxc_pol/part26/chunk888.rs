//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 888/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk888(t10305: f64, t794: f64, t188: f64, t297: f64, t818: f64, t2531: f64, t799: f64, t2493: f64, t435: f64, t3243: f64, t2316: f64, t493: f64) -> (f64, f64, f64, f64) {
    let t10306 = t794 * t10305;
    let t10309 = t188 * t818 * t297;
    let t10310 = t10309 * t2531;
    let t10311 = t799 * t10310;
    let t10313 = t435 * t2493;
    let t10314 = t3243 * t10313;
    let t10316 = t493 * t2316;
    (t10306, t10311, t10314, t10316)
}
