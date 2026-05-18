//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 939/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk939<F: Float>(t2026: F, t2212: F, t191: F, t214: F, t3: F, t675: F, t13: F, t2969: F, t6429: F) -> (F, F, F, F) {
    let t8518 = t2212 * t2026;
    let t8519 = t8518 * t191;
    let t8520 = t214 * t3;
    let t8521 = t8520 * t675;
    let t8526 = t6429 * t13 * t2969;
    (t8518, t8519, t8521, t8526)
}
