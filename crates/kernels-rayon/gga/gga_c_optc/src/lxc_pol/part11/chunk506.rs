//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 506/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk506(t1313: f64, t1924: f64, t193: f64, t1303: f64, t714: f64, t1378: f64, t24: f64, t862: f64, t2548: f64, t322: f64) -> (f64, f64, f64, f64) {
    let t3573 = t193 * t1924 * t1313;
    let t3593 = t1303 * t714;
    let t3605 = t24 * t1378;
    let t3606 = t862 * t3605;
    let t3608 = t322 * t2548;
    (t3573, t3593, t3606, t3608)
}
