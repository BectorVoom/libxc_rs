//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 559/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk559<F: Float>(t1036: F, t2633: F, t2632: F, t236: F, t435: F, t438: F, t14: F, t32: F, t2223: F, t16: F, t1846: F, t1022: F, t15: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2634 = t2633 * t1036;
    let t2636 = 2.0 * t2632 * t2634;
    let t2639 = 1.0 / t438 / t435 * t236;
    let t2640 = t32 * t14;
    let t2641 = t2640 * t2223;
    let t2642 = t2639 * t2641;
    let t2644 = t16 * t1846;
    let t2645 = t1022 * t2644;
    let t2647 = t15 * t1846;
    (t2634, t2636, t2639, t2640, t2641, t2642, t2644, t2645, t2647)
}
