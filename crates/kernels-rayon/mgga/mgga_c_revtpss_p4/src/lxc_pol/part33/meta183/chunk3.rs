//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 876/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk876(t1043: f64, t3154: f64, t4893: f64, t3117: f64, t3317: f64, t4891: f64) -> (f64, f64, f64, f64) {
    let t4894 = t3154 * t1043;
    let t4895 = t4893 * t4894;
    let t4896 = t3117 * t4895;
    let t4899 = t3317 * t4891;
    (t4894, t4895, t4896, t4899)
}
