//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 482/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk482<F: Float>(t2189: F, t2236: F, t2234: F, t2164: F, t2167: F, t2178: F, t816: F, t820: F) -> (F, F, F, F, F) {
    let t2237 = t2189 * t2236;
    let t2239 = F::cast_from(0.16081979498692535067e2_f64) * t2234 * t2237;
    let t2240 = F::cast_from(0.22831111111111111111e-1_f64) * t2164;
    let t2243 = t2240 - F::cast_from(0.34246666666666666666e-1_f64) * t2167 + F::new(0.5137e-1) * t2178;
    let t2246 = t816 * t820;
    (t2237, t2239, t2240, t2243, t2246)
}
