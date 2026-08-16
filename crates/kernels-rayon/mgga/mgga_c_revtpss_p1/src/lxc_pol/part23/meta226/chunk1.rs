//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1327/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1327(t3433: f64, t6474: f64, t3439: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1744: f64, t1169: f64) -> (f64, f64, f64, f64) {
    let t6476 = 0.16081979498692535067e2_f64 * t3433 * t6474;
    let t6481 = t3439 - 0.11415555555555555555e-1_f64 * t5044 - 0.11415555555555555555e-1_f64 * t6423 + 0.34246666666666666666e-1_f64 * t6427 + 0.17123333333333333333e-1_f64 * t6431;
    let t6486 = t1744 * t1744;
    let t6487 = t6486 * t1169;
    (t6476, t6481, t6486, t6487)
}
