//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 489/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk489(t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64, t828: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t225: f64, t1204: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3717 = t1209 * t1284;
    let t3718 = t3717 * t3624;
    let t3719 = t66 * t482;
    let t3720 = t828 * t3719;
    let t3732 = t460 * t1269;
    let t3736 = 1.0_f64 / t1275 / t493;
    let t3737 = t225 * t3736;
    let t3746 = t1204 * t1284;
    (t3718, t3719, t3720, t3732, t3737, t3746)
}
