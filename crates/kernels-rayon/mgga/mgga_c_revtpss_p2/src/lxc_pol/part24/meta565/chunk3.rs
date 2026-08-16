//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1717/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1717(t1024: f64, t1082: f64, t1089: f64, t12149: f64, t1685: f64, t1692: f64, t19569: f64, t19608: f64, t23959: f64, t24084: f64, t24123: f64, t24126: f64, t3287: f64, t55599: f64, t55747: f64, t55887: f64, t79863: f64, t88885: f64, t89035: f64, t89158: f64, t89240: f64) -> f64 {
    let t89632 = 0.26341796731742046395e1_f64 * t23959 * t1692 - 0.26341796731742046395e1_f64 * t79863 * t1685 - 0.79025390195226139184e1_f64 * t19608 * t24084 - 0.26341796731742046395e1_f64 * t3287 * t88885 * t1089 - 0.39512695097613069592e1_f64 * t3287 * t89240 * t1089 - 0.79025390195226139184e1_f64 * t19569 * t24084 + 0.26341796731742046395e1_f64 * t55599 * t24123 - 0.65854491829355115987e0_f64 * t1024 * t1082 * t89158 + 0.15805078039045227836e2_f64 * t55747 * t24126 + 0.15805078039045227836e2_f64 * t55887 * t24126 + 0.79025390195226139183e1_f64 * t12149 * t89035 * t1089;
    t89632
}
