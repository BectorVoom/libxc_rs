//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1029/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1029<F: Float>(t2012: F, t5639: F, t4416: F, t5638: F, t822: F, t2021: F, t7512: F, t200: F, t4598: F) -> (F, F, F, F) {
    let t14549 = t2012 * t5639;
    let t14555 = t822 * t5638 * t4416;
    let t14571 = t2021 * t7512;
    let t14626 = t4598 * t200;
    (t14549, t14555, t14571, t14626)
}
