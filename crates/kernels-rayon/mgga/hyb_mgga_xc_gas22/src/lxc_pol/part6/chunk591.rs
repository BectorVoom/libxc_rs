//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 591/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk591(t2636: f64, t2639: f64, t1075: f64, t1082: f64, t1090: f64, t1097: f64, t221: f64, t222: f64, t2662: f64, t2705: f64, t2716: f64, t2726: f64, t2734: f64, t2762: f64, t2766: f64, t2772: f64, t2774: f64, t2784: f64, t2789: f64, t2792: f64, t2798: f64, t2802: f64, t2803: f64, t2806: f64, t2809: f64, t479: f64, t492: f64) -> (f64, f64) {
    let t2810 = t2636 * t2639;
    let t2813 = -0.70983522622222222221e-3_f64 * t221 * t2662 * t479 - 0.34246666666666666666e-1_f64 * t222 * t2766 * t1082 - 2.0_f64 * t2772 * t2774 + 1.0_f64 * t1075 * t2784 + 0.32163958997385070134e2_f64 * t2789 * t2792 + t2762 + t2705 + t2716 - t2726 - t2734 - 0.24415263074675393405e-3_f64 * t221 * t2662 * t492 - 0.10843581300301739842e-1_f64 * t222 * t2798 * t1097 - 0.11696447245269292414e1_f64 * t2802 * t2803 + 0.5848223622634646207e0_f64 * t1090 * t2806 + 0.17315859105681463759e2_f64 * t2809 * t2810;
    (t2810, t2813)
}
