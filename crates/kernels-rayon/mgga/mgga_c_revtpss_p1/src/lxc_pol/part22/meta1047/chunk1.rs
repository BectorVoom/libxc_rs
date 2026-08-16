//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3679/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679(t1149: f64, t3433: f64, t69591: f64, t1170: f64, t1187: f64, t12486: f64, t12491: f64, t17150: f64, t1757: f64, t20537: f64, t20665: f64, t20668: f64, t20678: f64, t3496: f64, t3497: f64, t3515: f64, t45064: f64, t45177: f64, t6538: f64, t69565: f64, t69569: f64, t69571: f64, t69573: f64, t69575: f64, t69577: f64, t69579: f64, t69581: f64, t69583: f64, t69585: f64, t69587: f64, t69590: f64) -> (f64, f64) {
    let t69594 = 0.32163958997385070134e2_f64 * t3433 * t69591 * t1149;
    let t69595 = -0.23392894490538584828e1_f64 * t3496 * t1757 * t17150 - 0.20779030926817756511e3_f64 * t45064 * t20665 - 0.10389515463408878255e3_f64 * t12486 * t6538 * t3515 - 0.12304822629859687989e5_f64 * t45177 * t20678 * t3497 - 0.23392894490538584828e1_f64 * t12491 * t20668 - 0.23392894490538584828e1_f64 * t3496 * t20537 * t1187 + 2.0_f64 * t69565 * t1170 - t69569 + t69571 - t69573 + t69575 - t69577 - t69579 + t69581 + t69583 + t69585 - t69587 + t69590 - t69594;
    (t69594, t69595)
}
