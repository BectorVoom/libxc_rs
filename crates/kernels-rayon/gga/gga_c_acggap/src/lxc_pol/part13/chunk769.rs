//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 769/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk769(t2140: f64, t7990: f64, t609: f64, t879: f64, t2132: f64, t2138: f64, t847: f64, t2131: f64, t119: f64, t2122: f64, t159: f64, t3874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7991 = t7990 * t2140;
    let t7993 = t609 * t879;
    let t7994 = t2132 * t7993;
    let t7996 = 0.8673628188205199462e0_f64 * t2138 * t7994;
    let t7997 = t609 * t847;
    let t7998 = t2132 * t7997;
    let t8000 = 0.8673628188205199462e0_f64 * t2131 * t7998;
    let t8001 = t119 * t2122;
    let t8004 = t3874 * t159;
    (t7991, t7993, t7994, t7996, t7997, t7998, t8000, t8001, t8004)
}
