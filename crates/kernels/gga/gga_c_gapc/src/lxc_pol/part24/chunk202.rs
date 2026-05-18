//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 202/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk202<F: Float>(t255: F, t256: F, t62: F, t1: F, t252: F, t348: F, t352: F, t354: F, t14: F, t351: F) -> (F, F, F, F, F) {
    let t737 = F::new(1.0) / t256 / t255;
    let t738 = t62 * t737;
    let t740 = t348 * t252 * t1;
    let t745 = -F::new(0.14921166666666666667e-3) * t352 - F::new(0.39332083333333333333e-2) * t354;
    let t748 = -t740 * t351 / F::new(12.0) + t14 * t745 / F::new(2.0);
    (t737, t738, t740, t745, t748)
}
