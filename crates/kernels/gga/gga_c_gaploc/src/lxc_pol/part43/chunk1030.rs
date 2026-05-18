//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1030/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1030<F: Float>(t42380: F, t42381: F, t42385: F, t42388: F, t42390: F, t42392: F, t42395: F, t42398: F, t42401: F, t42405: F, t42407: F, t42413: F, t42421: F, t48178: F, t48182: F, t48185: F, t48188: F, t48191: F, t48194: F, t48198: F) -> F {
    let t50917 = -F::new(0.38342925953920749676e0) * t48178 - t48182 + t48185 - t42380 + t42381 - t42385 + t42388 - t42390 + t42392 - t42395 - t42398 - F::new(0.21450293971110256002e1) * t48188 - F::new(0.21450293971110256002e1) * t48191 - F::new(0.51123901271894332901e0) * t48194 + t48198 - t42401 - t42405 + t42407 - t42413 - t42421;
    t50917
}
