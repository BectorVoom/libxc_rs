//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 531/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk531<F: Float>(t2475: F, t2519: F, t2517: F, t2450: F, t2453: F, t2464: F, t963: F, t967: F) -> (F, F, F, F, F) {
    let t2520 = t2475 * t2519;
    let t2522 = 0.16081979498692535067e2 * t2517 * t2520;
    let t2523 = 0.22831111111111111111e-1 * t2450;
    let t2526 = t2523 - 0.34246666666666666666e-1 * t2453 + 0.5137e-1 * t2464;
    let t2529 = t963 * t967;
    (t2520, t2522, t2523, t2526, t2529)
}
