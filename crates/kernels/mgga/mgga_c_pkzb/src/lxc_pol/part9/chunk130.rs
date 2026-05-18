//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 130/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk130<F: Float>(t369: F, t378: F, t237: F, t354: F, t356: F, t365: F, t23: F, t275: F) -> (F, F, F) {
    let t379 = t369 * t378;
    let t382 = t237 * (-F::new(0.310907e-1) * t356 * t365 + t354 - F::new(0.19751673498613801407e-1) * t379);
    let t384 = F::new(0.19751673498613801407e-1) * t237 * t379;
    let t385 = t23 * t275;
    (t382, t384, t385)
}
