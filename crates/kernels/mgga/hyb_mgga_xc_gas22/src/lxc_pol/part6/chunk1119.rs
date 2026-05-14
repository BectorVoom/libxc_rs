//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1119/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1119<F: Float>(t1057: F, t7483: F, t1052: F, t7313: F, t1048: F, t2712: F, t7435: F, t2696: F, t2813: F, t462: F, t2630: F, t2649: F, t2662: F, t222: F, t567: F, t7440: F, t7444: F) -> (F, F, F, F, F, F) {
    let t21975 = 16.0 * t1057 * t7483;
    let t21978 = t1052 * t7313;
    let t21982 = 8.0 * t2712 * t1048 * t7435;
    let t21984 = t462 * t2696 * t2813;
    let t21990 = 0.86748650402413918736e-1 * t2630 * t2662 * t2649;
    let t21994 = 0.3684616320282908548e2 * t222 * t567 * t7440 * t7444;
    (t21975, t21978, t21982, t21984, t21990, t21994)
}
