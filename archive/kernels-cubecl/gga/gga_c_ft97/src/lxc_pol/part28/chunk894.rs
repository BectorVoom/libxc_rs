//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 894/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk894<F: Float>(t35138: F, t35148: F, t143: F, t160: F, t1384: F, t6615: F, t574: F, t605: F, t1359: F, t6718: F, t1017: F, t7414: F) -> (F, F, F, F, F, F, F) {
    let t35149 = t35138 + t35148;
    let t35151 = t143 * t35149 * t160;
    let t35155 = t6615 * t1384;
    let t35157 = t574 * t605 * t35155;
    let t35160 = t1359 * t6718;
    let t35162 = t574 * t605 * t35160;
    let t35166 = t574 * t7414 * t1017;
    (t35149, t35151, t35155, t35157, t35160, t35162, t35166)
}
