//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 591/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk591<F: Float>(t1016: F, t2355: F, t921: F) -> (F, F) {
    let t3365 = t2355 * t1016;
    let t3366 = t1016 * t921;
    (t3365, t3366)
}
