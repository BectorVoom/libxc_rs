//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1896/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1896(t19639: f64, t6271: f64, t3117: f64, t4786: f64, t6100: f64, t3092: f64, t1065: f64, t6244: f64, t906: f64, t1042: f64, t3172: f64, t6301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19640 = t6271 * t19639;
    let t19641 = t3117 * t19640;
    let t19644 = t6100 * t4786;
    let t19645 = t3092 * t19644;
    let t19649 = t1065 * t6244;
    let t19650 = t19649 * t906;
    let t19651 = t1042 * t19650;
    let t19658 = t3172 * t6301;
    (t19640, t19641, t19644, t19645, t19649, t19650, t19651, t19658)
}
