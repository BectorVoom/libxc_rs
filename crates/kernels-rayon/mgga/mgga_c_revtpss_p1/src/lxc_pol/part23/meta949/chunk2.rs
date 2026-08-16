//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3139/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3139(t13042: f64, t24663: f64, t3172: f64, t5284: f64, t6587: f64, t1774: f64, t20900: f64, t606: f64) -> (f64, f64, f64, f64) {
    let t82469 = t13042 * t3172 * t24663;
    let t82471 = t6587 * t5284;
    let t82476 = t1774 * t20900;
    let t82481 = t1774 * t606;
    (t82469, t82471, t82476, t82481)
}
