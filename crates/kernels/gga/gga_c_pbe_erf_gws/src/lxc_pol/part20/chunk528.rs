//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 528/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk528<F: Float>(t2784: F, t598: F, t186: F, t185: F, t1004: F, t172: F, t184: F) -> (F, F, F, F, F) {
    let t2785 = t598 * t2784;
    let t2786 = t186 * t2785;
    let t2788 = 2.0 / 15.0 * t185 * t2786;
    let t2789 = t172 * t1004;
    let t2790 = t2789 * t184;
    (t2785, t2786, t2788, t2789, t2790)
}
