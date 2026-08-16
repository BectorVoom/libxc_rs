//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 980/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk980(t2274: f64, t2643: f64, t2264: f64, t123: f64, t2606: f64, t2673: f64, t311: f64, t7856: f64, t140: f64, t309: f64, t883: f64, t3832: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10913 = t2643 * t2274;
    let t10918 = t2643 * t2264;
    let t10925 = t2606 * t123;
    let t10926 = t2673 * t10925;
    let t10935 = t311 * t7856;
    let t10952 = t883 * t309 * t140;
    let t10953 = t3832 * t10952;
    (t10913, t10918, t10925, t10926, t10935, t10953)
}
