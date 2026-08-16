//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1026/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1026<F: Float>(t10846: F, t10874: F, t10902: F, t10945: F, t10991: F, t11036: F, t11087: F, t11122: F, t3511: F, t841: F, t1052: F, t7822: F) -> (F, F, F) {
    let t11125 = t10846 + t10874 + t10902 + t10945 + t10991 + t11036 + t11087 + t11122;
    let t11127 = t3511 * t841;
    let t11130 = t7822 * t1052;
    (t11125, t11127, t11130)
}
