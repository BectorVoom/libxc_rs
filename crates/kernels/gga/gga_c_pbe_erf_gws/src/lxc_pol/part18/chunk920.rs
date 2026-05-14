//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 920/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk920<F: Float>(t3886: F, t938: F, t2409: F, t3067: F, t3742: F, t6781: F, t3703: F, t810: F) -> (F, F, F, F) {
    let t11354 = t3886 * t938;
    let t11356 = t2409 * t3067 * t11354;
    let t11360 = t2409 * t6781 * t3742;
    let t11363 = t3703 * t810;
    (t11354, t11356, t11360, t11363)
}
