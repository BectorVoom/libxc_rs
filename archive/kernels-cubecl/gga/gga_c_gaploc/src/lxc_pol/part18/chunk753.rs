//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 753/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk753<F: Float>(t169: F, t299: F, t7112: F, t706: F, t739: F, t738: F, t278: F, t481: F, t5286: F) -> (F, F, F, F) {
    let t7114 = t7112 * t169 * t299;
    let t7115 = t706 * t7114;
    let t7124 = t739 * t7112;
    let t7125 = t738 * t7124;
    let t7129 = t481 * t5286 * t278;
    (t7115, t7124, t7125, t7129)
}
