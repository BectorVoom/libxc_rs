//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 591/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk591<F: Float>(t2636: F, t2639: F, t1075: F, t1082: F, t1090: F, t1097: F, t221: F, t222: F, t2662: F, t2705: F, t2716: F, t2726: F, t2734: F, t2762: F, t2766: F, t2772: F, t2774: F, t2784: F, t2789: F, t2792: F, t2798: F, t2802: F, t2803: F, t2806: F, t2809: F, t479: F, t492: F) -> (F, F) {
    let t2810 = t2636 * t2639;
    let t2813 = -F::cast_from(0.70983522622222222221e-3_f64) * t221 * t2662 * t479 - F::cast_from(0.34246666666666666666e-1_f64) * t222 * t2766 * t1082 - F::new(2.0) * t2772 * t2774 + F::new(1.0) * t1075 * t2784 + F::cast_from(0.32163958997385070134e2_f64) * t2789 * t2792 + t2762 + t2705 + t2716 - t2726 - t2734 - F::cast_from(0.24415263074675393405e-3_f64) * t221 * t2662 * t492 - F::cast_from(0.10843581300301739842e-1_f64) * t222 * t2798 * t1097 - F::cast_from(0.11696447245269292414e1_f64) * t2802 * t2803 + F::cast_from(0.5848223622634646207e0_f64) * t1090 * t2806 + F::cast_from(0.17315859105681463759e2_f64) * t2809 * t2810;
    (t2810, t2813)
}
