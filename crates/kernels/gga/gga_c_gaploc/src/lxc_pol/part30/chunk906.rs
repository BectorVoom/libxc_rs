//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 906/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk906<F: Float>(t10667: F, t739: F, t738: F, t169: F, t299: F, t706: F, t325: F, t3431: F) -> (F, F, F, F, F) {
    let t10668 = t739 * t10667;
    let t10669 = t738 * t10668;
    let t10673 = t10667 * t169 * t299;
    let t10674 = t706 * t10673;
    let t10677 = t325 * t3431;
    (t10668, t10669, t10673, t10674, t10677)
}
