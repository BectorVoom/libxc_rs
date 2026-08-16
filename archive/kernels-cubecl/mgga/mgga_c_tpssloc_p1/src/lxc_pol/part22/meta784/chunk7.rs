//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2698/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698<F: Float>(t19924: F, t19994: F, t39585: F, t39590: F, t39593: F, t39595: F, t5122: F, t5126: F, t54431: F, t54436: F, t74484: F, t74485: F, t74486: F) -> F {
    let t75237 = F::cast_from(36.0_f64) * t19924 * t5122 * t5126 + F::cast_from(18.0_f64) * t19994 * t5122 * t5126 - t39585 + t39590 - t39593 + t39595 - t54431 - t54436 + t74484 + t74485 - t74486;
    t75237
}
