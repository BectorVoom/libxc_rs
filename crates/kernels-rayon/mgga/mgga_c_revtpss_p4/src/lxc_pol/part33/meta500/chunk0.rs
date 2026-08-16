//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1807/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1807(t3596: f64, t3598: f64, t3594: f64, t7616: f64, t1230: f64, t7623: f64, t3636: f64, t7624: f64, t3704: f64, t7618: f64, t479: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26842 = t3596 * sigma2;
    let t26843 = t26842 * t3598;
    let t26844 = t3594 * t26843;
    let t26848 = t7616 * t3598;
    let t26849 = t3594 * t26848;
    let t26852 = t1230 * t7623;
    let t26855 = t7624 * t3636;
    let t26863 = t7618 * t3704;
    let t26865 = sigma2 * t479;
    (t26843, t26844, t26848, t26849, t26852, t26855, t26863, t26865)
}
