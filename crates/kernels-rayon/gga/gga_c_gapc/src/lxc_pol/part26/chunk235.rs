//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 235/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk235(t291: f64, t597: f64, t906: f64, t315: f64, t604: f64, t181: f64, t820: f64, t311: f64, t825: f64) -> (f64, f64, f64, f64) {
    let t907 = t597 * t291 * t906;
    let t910 = t604 * t315;
    let t913 = t181 * t820;
    let t916 = t311 * t825;
    (t907, t910, t913, t916)
}
