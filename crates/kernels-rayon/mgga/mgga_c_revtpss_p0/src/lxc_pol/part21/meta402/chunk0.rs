//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1863/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1863(t12987: f64, t480: f64, t12629: f64, t482: f64, t371: f64, t372: f64, t127: f64, t3672: f64, t3671: f64, t140: f64, t3693: f64, t1222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12988 = t12987 * t480;
    let t12989 = t482 * t12629;
    let t12991 = t371 * t372 * t12989;
    let t12995 = t371 * t127 * t3672;
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    (t12988, t12989, t12991, t12995, t12996, t12998, t12999)
}
