//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1894/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1894(t19620: f64, t6271: f64, t3117: f64, t19501: f64, t3095: f64, t3092: f64, t1043: f64, t3155: f64) -> (f64, f64, f64, f64, f64) {
    let t19621 = t6271 * t19620;
    let t19622 = t3117 * t19621;
    let t19625 = t19501 * t3095;
    let t19626 = t3092 * t19625;
    let t19634 = t3155 * t1043;
    (t19621, t19622, t19625, t19626, t19634)
}
