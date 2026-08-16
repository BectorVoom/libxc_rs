//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1160/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1160(t22026: f64, t2661: f64, t94550: f64, t22056: f64, t25972: f64, t22021: f64, t25986: f64, t22068: f64, t25978: f64, t6880: f64, t6856: f64, t1927: f64, t5816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108604 = t2661 * t94550 * t22026;
    let t108608 = t25972 * t22056;
    let t108623 = t2661 * t25986 * t22021;
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    let t108629 = t25978 * t6856;
    let t108879 = t1927 * t5816;
    (t108604, t108608, t108623, t108625, t108627, t108629, t108879)
}
