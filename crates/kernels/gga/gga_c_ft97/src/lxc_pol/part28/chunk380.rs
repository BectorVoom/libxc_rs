//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 380/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk380<F: Float>(t167: F, t2185: F, t5860: F, t1359: F, t574: F, t616: F, t609: F, t605: F, t5842: F, t1380: F, t376: F, t89: F, t1391: F, t379: F, t569: F, t558: F) -> (F, F, F, F, F, F, F, F) {
    let t5862 = t2185 * t167 * t5860;
    let t5866 = t574 * t616 * t1359;
    let t5869 = t1359 * t609;
    let t5871 = t574 * t605 * t5869;
    let t5875 = t574 * t167 * t5842;
    let t5880 = t89 * t376 * t1380 / 9.0;
    let t5882 = t569 * t1391 * t379;
    let t5886 = t574 * t1391 * t558;
    (t5862, t5866, t5869, t5871, t5875, t5880, t5882, t5886)
}
