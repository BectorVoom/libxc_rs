//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 762/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk762<F: Float>(t1764: F, t187: F, t22: F, t1679: F, t586: F, t1878: F, t1648: F, t1652: F, t1683: F, t633: F, t1725: F, t582: F) -> (F, F, F, F, F, F) {
    let t5292 = F::new(1.0) / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5304 = t1679 * t586;
    let t5312 = t1878 * t586;
    let t5315 = t1648 * t1652;
    let t5317 = t633 * t1683;
    let t5322 = t582 * t1725;
    (t5293, t5304, t5312, t5315, t5317, t5322)
}
