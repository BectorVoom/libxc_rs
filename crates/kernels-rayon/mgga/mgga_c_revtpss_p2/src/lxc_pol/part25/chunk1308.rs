//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1308/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1308(t26028: f64, t9958: f64, t7262: f64, t820: f64, t844: f64, t3940: f64, t27940: f64, t9837: f64, t9842: f64, t9832: f64, t9828: f64, t25983: f64, t9914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94426 = t26028 * t9958;
    let t94429 = t820 * t7262 * t844;
    let t94430 = t94429 * t3940;
    let t94432 = t27940 * t9837;
    let t94434 = t27940 * t9842;
    let t94436 = t26028 * t9832;
    let t94438 = t26028 * t9828;
    let t94440 = t25983 * t9914;
    (t94426, t94430, t94432, t94434, t94436, t94438, t94440)
}
