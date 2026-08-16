//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 857/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk857(t1210: f64, t618: f64, t2137: f64, t441: f64, t7923: f64, t615: f64, t2130: f64, t7922: f64, t861: f64, t14651: f64, t159: f64, t448: f64, t7911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29984 = t1210 * t618;
    let t29985 = t2137 * t29984;
    let t29991 = t7923 * t441;
    let t29994 = t615 * t29984;
    let t30005 = t7923 * t2130;
    let t30009 = t7922 * t861 * t2130;
    let t30023 = t14651 * t159;
    let t30028 = t7911 * t448;
    (t29984, t29985, t29991, t29994, t30005, t30009, t30023, t30028)
}
