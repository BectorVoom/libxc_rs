//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 994/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk994<F: Float>(t9011: F, t7021: F, t7035: F, t7037: F, t7040: F, t7043: F, t9008: F, t9029: F, t9149: F, t9152: F, t9155: F, t9159: F) -> (F, F) {
    let t9235 = F::cast_from(0.103295e1_f64) * t9011;
    let t9240 = -F::cast_from(0.3529725e1_f64) * t9149 + F::cast_from(0.264729375e1_f64) * t9152 - F::cast_from(0.157790625e0_f64) * t9155 + F::cast_from(0.68863333333333333333e0_f64) * t9008 + F::cast_from(0.34731666666666666667e0_f64) * t9159 - t9235 + F::cast_from(0.1549425e1_f64) * t9029 - t7021 - t7035 + F::cast_from(0.69463333333333333333e0_f64) * t7037 - F::cast_from(0.20839e0_f64) * t7040 - F::cast_from(0.20839e0_f64) * t7043;
    (t9235, t9240)
}
