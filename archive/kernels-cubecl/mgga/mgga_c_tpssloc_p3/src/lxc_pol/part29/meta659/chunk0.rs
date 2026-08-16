//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2187/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2187<F: Float>(t1799: F, t3886: F, t22633: F, t22635: F, t3888: F, t80663: F, t80671: F, t1887: F, t80827: F, t26334: F, t26339: F, t81159: F) -> (F, F, F, F, F, F) {
    let t90488 = t3886 * t1799;
    let t90491 = t22633 * t22635 * t90488 * t3888;
    let t90493 = F::cast_from(0.12793931631041761173e0_f64) * t80663;
    let t90496 = F::cast_from(0.10417915756705434098e0_f64) * t80671;
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    let t90500 = t81159 * t26339;
    (t90491, t90493, t90496, t90497, t90498, t90500)
}
