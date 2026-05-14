//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 723/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk723<F: Float>(t2409: F, t835: F, t882: F, t1882: F, t2854: F, t2749: F, t2801: F, t296: F, t192: F, t7640: F, t10262: F, t319: F, t2842: F, t863: F, t2844: F, t2751: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10675 = t835 * t882 * t2409;
    let t10678 = t1882 * t2854;
    let t10680 = t2749 * t2801;
    let t10681 = t296 * t10680;
    let t10683 = t192 * t7640;
    let t10685 = t10683 * t319 * t10262;
    let t10688 = t863 * t2842;
    let t10689 = t10688 * t2844;
    let t10690 = t296 * t10689;
    let t10693 = t1882 * t2751;
    (t10675, t10678, t10680, t10681, t10683, t10685, t10688, t10689, t10690, t10693)
}
