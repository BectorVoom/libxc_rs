//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 559/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk559<F: Float>(t221: F, t2662: F, t439: F, t236: F, t2656: F, t1040: F, t2659: F, t14: F, t2212: F, t237: F, t2657: F, t2660: F) -> (F, F, F, F, F, F) {
    let t2663 = t221 * t2662;
    let t2665 = 1.0/f64::sqrt(t439);
    let t2666 = t2665 * t236;
    let t2667 = t2666 * t2656;
    let t2669 = t1040 * t2659;
    let t2672 = t237 * t14 * t2212;
    let t2674 = -0.57538888888888888889e0 * t2657 + 0.11507777777777777778e1 * t2660 + 0.40256666666666666667e0 * t2663 + 0.366775e-1 * t2667 + 0.73355e-1 * t2669 + 0.137975e0 * t2672;
    (t2663, t2666, t2667, t2669, t2672, t2674)
}
