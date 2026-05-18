//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1187/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1187<F: Float>(t222: F, t567: F, t7440: F, t7444: F, t1057: F, t7313: F, t2643: F, t7242: F, t1068: F, t7544: F, t1097: F, t1110: F, t2647: F, t7410: F) -> (F, F, F, F, F) {
    let t21994 = F::new(0.3684616320282908548e2) * t222 * t567 * t7440 * t7444;
    let t21997 = t1057 * t7313;
    let t21999 = t2643 * t7242;
    let t22004 = t7544 * t1068;
    let t22009 = F::new(0.46785788981077169656e1) * t1110 * t2647 * t7410 * t1097;
    (t21994, t21997, t21999, t22004, t22009)
}
