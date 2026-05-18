//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 227/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk227<F: Float>(t122: F, t2378: F, t200: F, t223: F, t677: F, t695: F, t194: F, t195: F, t25: F, t1636: F, t191: F) -> (F, F, F, F, F, F, F) {
    let t2379 = t122 * t2378;
    let t2382 = t200 * t200;
    let t2383 = t2382 * t223;
    let t2387 = t677 * t695;
    let t2393 = F::new(1.0) / t195 / t194;
    let t2394 = t25 * t2393;
    let t2399 = t1636 * t191;
    (t2379, t2382, t2383, t2387, t2393, t2394, t2399)
}
