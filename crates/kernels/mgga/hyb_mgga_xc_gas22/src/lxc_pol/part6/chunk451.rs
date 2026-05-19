//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 451/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk451<F: Float>(t143: F, t2054: F, t39: F, t2028: F, t2052: F, t2022: F, t699: F, t2002: F, t702: F, t2047: F, t2048: F, t572: F, t147: F, t168: F) -> (F, F, F, F, F, F, F, F) {
    let t145 = F::new(0.135e1) < t143;
    let t2055 = t39 * t2054;
    let t2057 = t2052 * t2055 * t2028;
    let t2060 = t39 * t2022;
    let t2062 = t699 * t2060 * t2028;
    let t2066 = t699 * t702 * t2002;
    let t2069 = t2047 + t2048 / F::new(81.0) - t572 * t2057 / F::new(81.0) + t572 * t2062 / F::new(27.0) - t572 * t2066 / F::new(54.0);
    let t2070 = piecewise3::<F>(t145, t2069, F::new(0.0));
    let t2098 = F::new(1.0) / t168 / t147;
    (t2055, t2057, t2060, t2062, t2066, t2069, t2070, t2098)
}
