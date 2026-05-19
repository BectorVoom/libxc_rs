//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 21/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk21<F: Float>(t46: F, t55: F, t44: F, t2: F, t3: F) -> (F, F, F, F) {
    let t56 = t46 * t55;
    let t58 = F::cast_from(0.19751789702565206229e-1_f64) * t44 * t56;
    let t59 = t3 * t2;
    let t60 = F::new(1.0) / t59;
    (t56, t58, t59, t60)
}
