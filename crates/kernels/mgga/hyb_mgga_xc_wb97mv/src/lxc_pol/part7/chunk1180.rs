//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1180/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1180<F: Float>(t2025: F, t683: F, t8563: F, t3300: F, t6715: F, t8567: F, t2035: F, t6725: F, t8571: F, t21309: F, t1323: F, t222: F, t6129: F) -> (F, F, F, F, F, F) {
    let t26241 = t683 * t2025 * t8563;
    let t26244 = t683 * t6715 * t3300;
    let t26247 = t683 * t2025 * t8567;
    let t26250 = t2035 * t6725 * t8571;
    let t26273 = 24.0 * t21309;
    let t26298 = t222 * t6129 * t1323;
    (t26241, t26244, t26247, t26250, t26273, t26298)
}
