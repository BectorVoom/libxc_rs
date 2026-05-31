//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 871/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk871<F: Float>(t2639: F, t7238: F, t7253: F, t1110: F, t2643: F, t2649: F, t1030: F, t1884: F, t1048: F, t222: F, t2711: F, t2714: F, t567: F) -> (F, F, F, F, F, F) {
    let t7255 = t7253 * t7238 * t2639;
    let t7257 = F::cast_from(0.10389515463408878255e3_f64) * t1110 * t7255;
    let t7258 = t2643 * t2649;
    let t7260 = t1884 * t1030;
    let t7263 = F::cast_from(0.71233333333333333332e-1_f64) * t222 * t7260 * t1048;
    let t7267 = F::cast_from(0.10685e0_f64) * t222 * t567 * t2711 * t2714;
    (t7255, t7257, t7258, t7260, t7263, t7267)
}
