//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 289/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk289<F: Float>(t3187: F, t3188: F, t1909: F, t100: F, t1780: F, t103: F, t1557: F, t942: F, t379: F, t1902: F, t432: F, t920: F) -> (F, F, F, F, F, F, F) {
    let t3189 = t3187 * t3188;
    let t3190 = t1909 * t3189;
    let t3193 = t1780 * t100;
    let t3194 = t103 * t1557;
    let t3195 = t3194 * t3188;
    let t3196 = t3193 * t3195;
    let t3199 = t103 * t942;
    let t3200 = t3199 * t379;
    let t3201 = t1902 * t3200;
    let t3204 = t920 * t432;
    (t3189, t3190, t3195, t3196, t3200, t3201, t3204)
}
