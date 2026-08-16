//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 640/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk640(t2655: f64, t2658: f64, t2669: f64, t2695: f64, t2828: f64, t2840: f64, t4038: f64, t4040: f64, t4041: f64, t4042: f64, t4043: f64, t4044: f64, t4046: f64, t4049: f64, t4050: f64, t4058: f64) -> f64 {
    let t5020 = t2828 + t2655 - t2658 + t4038 + t2840 + t4040 - t4041 - t4042 - t4043 - t4044 + t2669 + t2695 + t4046 - t4049 - t4050 - t4058;
    t5020
}
