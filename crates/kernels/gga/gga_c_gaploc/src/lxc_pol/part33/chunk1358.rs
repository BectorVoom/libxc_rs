//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1358/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1358<F: Float>(t12014: F, t29860: F, t29862: F, t29865: F, t29868: F, t31525: F, t31527: F, t31533: F, t31539: F, t31542: F, t31546: F, t31551: F, t31553: F, t31556: F, t4141: F) -> F {
    let t38295 = t31525 + t31527 + t31533 + t31539 + F::new(0.31616674039640166222e-2) * t4141 * t12014 + t31542 + t31546 - t31551 + t31553 - t31556 + t29860 - t29862 - t29865 - t29868;
    t38295
}
