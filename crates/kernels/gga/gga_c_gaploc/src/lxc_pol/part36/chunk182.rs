//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 182/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk182<F: Float>(t169: F, t874: F, t172: F, t452: F, t103: F, t78: F, t119: F, t481: F) -> (F, F, F, F) {
    let t875 = t874 * t169;
    let t876 = t875 * t172;
    let t877 = t452 * t876;
    let t880 = t78 * t103;
    let t882 = t481 * t880 * t119;
    (t876, t877, t880, t882)
}
