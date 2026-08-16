//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 428/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk428<F: Float>(t107: F, t9: F, t1559: F, t19: F, t1434: F, t584: F) -> (F, F, F) {
    let t4752 = t107 * t9;
    let t4779 = t1559 * t19;
    let t4780 = t4779 * t1434;
    let t4781 = t584 * t4780;
    (t4752, t4779, t4781)
}
