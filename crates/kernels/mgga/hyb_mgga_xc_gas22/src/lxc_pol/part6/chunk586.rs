//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 586/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk586<F: Float>(t1081: F, t2773: F, t2657: F, t2660: F, t2663: F, t2667: F, t2669: F, t2672: F) -> (F, F) {
    let t2774 = t2773 * t1081;
    let t2783 = -F::cast_from(0.78438333333333333333e0_f64) * t2657 + F::cast_from(0.15687666666666666667e1_f64) * t2660 + F::cast_from(0.68863333333333333333e0_f64) * t2663 + F::cast_from(0.14025833333333333333e0_f64) * t2667 + F::cast_from(0.28051666666666666667e0_f64) * t2669 + F::cast_from(0.17365833333333333333e0_f64) * t2672;
    (t2774, t2783)
}
