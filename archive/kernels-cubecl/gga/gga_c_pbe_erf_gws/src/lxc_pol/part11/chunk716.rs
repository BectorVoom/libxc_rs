//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 716/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk716<F: Float>(t3426: F, t395: F, t3430: F, t3584: F, t723: F, t3398: F, t586: F) -> (F, F, F, F) {
    let t10825 = t395 * t3426;
    let t10827 = t395 * t3430;
    let t10841 = t3584 * t723;
    let t10843 = t3398 * t586;
    (t10825, t10827, t10841, t10843)
}
