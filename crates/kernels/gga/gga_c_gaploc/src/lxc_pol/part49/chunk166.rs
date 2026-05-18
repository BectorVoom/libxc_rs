//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 166/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk166<F: Float>(t1: F, t296: F, t106: F, t787: F, t299: F, t550: F, t549: F, t121: F, t321: F) -> (F, F, F, F) {
    let t788 = t296 * t1;
    let t789 = t788 * t106;
    let t790 = t787 * t789;
    let t791 = t550 * t299;
    let t792 = t549 * t791;
    let t795 = t121 * t321;
    (t789, t790, t792, t795)
}
