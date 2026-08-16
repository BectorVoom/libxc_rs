//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3043/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043(t14939: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t14588: f64, t10518: f64, t14606: f64, t231: f64, t2782: f64, t2783: f64, t51380: f64) -> (f64, f64, f64, f64) {
    let t51505 = t689 * t869 * t233 * t14939;
    let t51507 = t10069 * t14588;
    let t51512 = t14606 * t10518;
    let t51519 = t2782 * t2783 * t51380 * t231;
    (t51505, t51507, t51512, t51519)
}
