//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 946/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk946(t11483: f64, t633: f64, t1688: f64, t5117: f64, t185: f64) -> (f64, f64, f64, f64) {
    let t11484 = t633 * t11483;
    let t11485 = t1688 * t5117;
    let t11486 = t11484 * t11485;
    let t11488 = t185 * t11483;
    (t11484, t11485, t11486, t11488)
}
