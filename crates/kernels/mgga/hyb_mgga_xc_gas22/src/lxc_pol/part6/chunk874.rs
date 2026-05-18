//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 874/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk874<F: Float>(t2639: F, t2674: F, t1095: F, t1097: F, t1096: F, t7238: F, t12: F, t442: F, t448: F, t461: F, t2655: F, t6610: F) -> (F, F, F, F, F, F, F) {
    let t7323 = t2674 * t2639;
    let t7324 = t7323 * t1095;
    let t7327 = t1097 * t2674;
    let t7330 = t7238 * t1096;
    let t7336 = F::new(1.0) / t442 / t448 * t12 / F::new(4.0);
    let t7337 = t7336 * t461;
    let t7339 = t2655 * t6610;
    (t7323, t7324, t7327, t7330, t7336, t7337, t7339)
}
