//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1159/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1159<F: Float>(t10290: F, t10304: F, t10298: F, t4349: F, t605: F, t1651: F, t3366: F, t27214: F, t6565: F, t6568: F, t8045: F, t1382: F, t3418: F) -> (F, F, F, F, F, F, F) {
    let t31454 = F::new(4.0) * t10290;
    let t31455 = F::new(2.0) * t10304;
    let t31458 = F::new(12.0) * t4349 * t10298 * t605;
    let t31461 = F::new(6.0) * t4349 * t3366 * t1651;
    let t31463 = F::new(6.0) * t27214 * t6565;
    let t31465 = F::new(4.0) * t8045 * t6568;
    let t31468 = F::new(2.0) * t1382 * t3418 * t1651;
    (t31454, t31455, t31458, t31461, t31463, t31465, t31468)
}
