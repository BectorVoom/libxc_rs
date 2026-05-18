//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1451/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1451<F: Float>(t12284: F, t12294: F, t32413: F, t32415: F, t32417: F, t32429: F, t32431: F, t32434: F, t32439: F, t32441: F, t32443: F, t32446: F, t32448: F, t32452: F, t32456: F, t7137: F) -> F {
    let t39425 = t32413 + t32415 - t32417 - t32429 + t32431 + t32434 - t32439 - t32441 + t32443 - t32446 - t32448 - t32452 + t32456 - F::new(0.61524209841137794271e-1) * t7137 * t12284 + F::new(0.41016139894091862847e-1) * t7137 * t12294;
    t39425
}
