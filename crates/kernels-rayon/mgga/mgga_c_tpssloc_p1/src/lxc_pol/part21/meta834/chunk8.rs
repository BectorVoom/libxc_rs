//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2961/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2961(t17884: f64, t3048: f64, t1046: f64, t10962: f64, t14085: f64, t14093: f64, t14491: f64, t1618: f64, t42570: f64, t4636: f64, t4641: f64, t4644: f64, t48430: f64, t48441: f64, t49866: f64, t5869: f64, t5875: f64, t61695: f64, t61699: f64, t61705: f64, t61708: f64, t61710: f64, t61713: f64) -> f64 {
    let t61715 = t3048 * t17884;
    let t61717 = t14085 * t4636 / 1152.0_f64 + t4644 * t14093 / 2304.0_f64 + t49866 * t1618 / 1536.0_f64 + t10962 * t5869 / 3072.0_f64 - t61695 / 432.0_f64 + t48430 / 648.0_f64 + t61699 / 432.0_f64 + t4641 * t14491 / 1536.0_f64 - t42570 * t5875 / 144.0_f64 + t61705 / 1152.0_f64 - t48441 / 54.0_f64 + t61708 / 3456.0_f64 - t61710 * t1046 / 432.0_f64 + t61713 / 2304.0_f64 - 5.0_f64 / 1944.0_f64 * t61715;
    t61717
}
