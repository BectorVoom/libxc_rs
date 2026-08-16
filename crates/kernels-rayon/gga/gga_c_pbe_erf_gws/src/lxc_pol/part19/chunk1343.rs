//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1343/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1343(t1109: f64, t1113: f64, t3972: f64, t3975: f64, t4408: f64, t2409: f64, t35910: f64, t3965: f64, t13808: f64, t15146: f64, t15191: f64, t50994: f64) -> (f64, f64, f64, f64) {
    let t57635 = t3972 * t3975 * t1113 * t4408 * t1109;
    let t57639 = t3965 * t2409 * t35910;
    let t57641 = t13808 * t15146;
    let t57643 = t50994 * t15191;
    (t57635, t57639, t57641, t57643)
}
