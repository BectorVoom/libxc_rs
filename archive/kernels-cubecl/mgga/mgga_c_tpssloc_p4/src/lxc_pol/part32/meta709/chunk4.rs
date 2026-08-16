//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2218/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2218<F: Float>(t25038: F, t25040: F, t86873: F, t17052: F, t6663: F, t82070: F, t82082: F, t86929: F, t92406: F, t98189: F, t98192: F, t98196: F, t98199: F, t98202: F) -> F {
    let t98205 = t25038 * t86873 * t25040;
    let t98208 = F::cast_from(0.16449340668482264365e-1_f64) * t98189 + t92406 + F::cast_from(0.3289868133696452873e-1_f64) * t98192 - t17052 * t6663 + t82070 + F::cast_from(0.3289868133696452873e-1_f64) * t98196 - F::cast_from(0.16449340668482264365e-1_f64) * t98199 - F::cast_from(0.16449340668482264365e-1_f64) * t98202 + F::cast_from(0.9869604401089358619e-1_f64) * t98205 - t86929 + F::cast_from(0.82246703342411321824e-2_f64) * t82082;
    t98208
}
