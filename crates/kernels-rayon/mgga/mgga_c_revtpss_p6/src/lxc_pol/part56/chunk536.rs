//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 536/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk536(t4314: f64, t4559: f64, t2: f64, t265: f64, t580: f64, t1593: f64, t689: f64) -> (f64, f64, f64) {
    let t4560 = t4314 + t4559;
    let t4567 = t265 * t2;
    let t4568 = t4567 * t580;
    let t4571 = t689 * t1593;
    (t4560, t4568, t4571)
}
