//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 103/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk103<F: Float>(t154: F, t277: F, t52: F, t273: F, t133: F, t158: F, t230: F, t265: F, t267: F) -> (F, F, F, F) {
    let t279 = t154 * t52 * t277;
    let t284 = F::cast_from(1.0_f64) / t273;
    let t285 = t133 * t284;
    let t287 = F::exp(-(-t230 + t265 + t267) * t158 * t285);
    (t279, t284, t285, t287)
}
