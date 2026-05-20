//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3679/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3679<F: Float>(t1149: F, t3433: F, t69591: F, t1170: F, t1187: F, t12486: F, t12491: F, t17150: F, t1757: F, t20537: F, t20665: F, t20668: F, t20678: F, t3496: F, t3497: F, t3515: F, t45064: F, t45177: F, t6538: F, t69565: F, t69569: F, t69571: F, t69573: F, t69575: F, t69577: F, t69579: F, t69581: F, t69583: F, t69585: F, t69587: F, t69590: F) -> (F, F) {
    let t69594 = F::cast_from(0.32163958997385070134e2_f64) * t3433 * t69591 * t1149;
    let t69595 = -F::cast_from(0.23392894490538584828e1_f64) * t3496 * t1757 * t17150 - F::cast_from(0.20779030926817756511e3_f64) * t45064 * t20665 - F::cast_from(0.10389515463408878255e3_f64) * t12486 * t6538 * t3515 - F::cast_from(0.12304822629859687989e5_f64) * t45177 * t20678 * t3497 - F::cast_from(0.23392894490538584828e1_f64) * t12491 * t20668 - F::cast_from(0.23392894490538584828e1_f64) * t3496 * t20537 * t1187 + F::new(2.0) * t69565 * t1170 - t69569 + t69571 - t69573 + t69575 - t69577 - t69579 + t69581 + t69583 + t69585 - t69587 + t69590 - t69594;
    (t69594, t69595)
}
