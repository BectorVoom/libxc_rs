//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2371/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371(t231: f64, t2782: f64, t2783: f64, t39709: f64, t10910: f64, t233: f64, t689: f64, t869: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64) -> (f64, f64, f64, f64) {
    let t40307 = t2782 * t2783 * t39709 * t231;
    let t40311 = t689 * t869 * t233 * t10910;
    let t40314 = 0.11564373972601816912e-1_f64 * t39515 * t2778;
    let t40316 = 0.56911289235245161963e-1_f64 * t39501 * t871;
    (t40307, t40311, t40314, t40316)
}
