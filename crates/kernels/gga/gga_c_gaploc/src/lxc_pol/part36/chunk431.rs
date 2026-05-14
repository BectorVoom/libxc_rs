//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 431/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk431<F: Float>(t1564: F, t911: F, t1415: F, t4390: F, t191: F, t599: F, t588: F) -> (F, F, F, F) {
    let t6915 = t911 * t1564;
    let t6963 = t1415 * t4390;
    let t6964 = t191 * t599;
    let t6985 = t588 * t599;
    (t6915, t6963, t6964, t6985)
}
