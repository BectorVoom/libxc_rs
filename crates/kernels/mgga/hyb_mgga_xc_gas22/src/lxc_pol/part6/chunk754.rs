//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 754/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk754<F: Float>(t143: F, t2052: F, t2055: F, t3925: F, t2060: F, t699: F, t3938: F, t702: F, t2047: F, t3169: F, t572: F) -> (F, F, F, F, F) {
    let t145 = F::cast_from(0.135e1_f64) < t143;
    let t4002 = t2052 * t2055 * t3925;
    let t4006 = t699 * t2060 * t3925;
    let t4010 = t699 * t702 * t3938;
    let t4013 = t2047 + t3169 / F::cast_from(81.0_f64) - t572 * t4002 / F::cast_from(81.0_f64) + t572 * t4006 / F::cast_from(27.0_f64) - t572 * t4010 / F::cast_from(54.0_f64);
    let t4014 = piecewise3::<F>(t145, t4013, F::cast_from(0.0_f64));
    (t4002, t4006, t4010, t4013, t4014)
}
