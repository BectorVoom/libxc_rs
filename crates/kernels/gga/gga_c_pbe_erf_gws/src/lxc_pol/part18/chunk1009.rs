//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1009/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1009<F: Float>(t274: F, t837: F, t850: F, t851: F, t833: F, t3955: F, t894: F, t2118: F, t332: F, t353: F, t4387: F, t859: F) -> (F, F, F, F, F, F) {
    let t14125 = t274 * t837;
    let t14127 = t850 * t851 * t14125;
    let t14128 = t14127 * t833;
    let t14130 = t3955 * t894;
    let t14135 = t2118 * t332;
    let t14138 = t859 * t353 * t4387;
    (t14125, t14127, t14128, t14130, t14135, t14138)
}
