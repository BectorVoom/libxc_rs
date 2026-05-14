//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 913/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk913<F: Float>(t481: F, t6212: F, t6211: F, t6480: F, t133: F, t2078: F, t255: F) -> (F, F, F, F) {
    let t6481 = t6212 * t481;
    let t6482 = t6211 * t6481;
    let t6483 = t6480 * t6482;
    let t6486 = t133 * t2078 * t255;
    (t6481, t6482, t6483, t6486)
}
