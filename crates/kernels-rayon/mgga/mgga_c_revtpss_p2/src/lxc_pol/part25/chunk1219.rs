//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1219/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1219(t5: f64, t92618: f64, t92649: f64, t92682: f64, t92715: f64, t117: f64, t25856: f64, t4254: f64, t13207: f64, t1936: f64, t651: f64, t2322: f64, t25851: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t92718 = piecewise3(t8, 0.0_f64, t92618 + t92649 + t92682 + t92715);
    let t92719 = t92718 * t117;
    let t92724 = 6.0_f64 * t4254 * t25856;
    let t92727 = 2.0_f64 * t651 * t13207 * t1936;
    let t92731 = 6.0_f64 * t2322 * t25851;
    (t92719, t92724, t92727, t92731)
}
