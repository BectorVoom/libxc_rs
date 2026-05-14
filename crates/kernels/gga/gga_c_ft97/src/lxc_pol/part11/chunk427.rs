//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 427/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk427<F: Float>(t2379: F, t2380: F, t200: F, t223: F, t237: F, t677: F, t695: F) -> (F, F, F, F) {
    let t2381 = t2379 * t2380;
    let t2382 = t200 * t200;
    let t2383 = t2382 * t223;
    let t2384 = t2383 * t237;
    let t2387 = t677 * t695;
    (t2381, t2382, t2384, t2387)
}
