//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 463/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk463<F: Float>(t7: F, t143: F, t172: F, t187: F, t2114: F, t2115: F, t2158: F, t739: F, t758: F, t139: F, t214: F, t26: F, t1877: F, t1847: F, t222: F, t226: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t144 = 0.135e1 <= t143;
    let t2162 = piecewise3(t144, t2114, -8.0 / 3.0 * t2115 * t187 - 16.0 / 3.0 * t739 * t758 - 8.0 / 3.0 * t172 * t2158);
    let t2163 = t139 * t2162;
    let t2164 = t2163 * t214;
    let t2165 = t26 * t2164;
    let t2170 = piecewise3(t8, 0.0, t1877);
    let t2175 = t222 * t1847 * t226;
    (t2162, t2163, t2164, t2165, t2170, t2175)
}
