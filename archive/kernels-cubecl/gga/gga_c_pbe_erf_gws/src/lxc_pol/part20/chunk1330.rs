//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1330/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1330<F: Float>(t11652: F, t14498: F, t14064: F, t3783: F, t11820: F, t14011: F, t11758: F, t4039: F, t11531: F, t14015: F, t8991: F, t9035: F) -> (F, F, F, F, F, F) {
    let t57011 = t14498 * t11652;
    let t57013 = t3783 * t14064;
    let t57015 = t14011 * t11820;
    let t57017 = t4039 * t11758;
    let t57019 = t14015 * t11531;
    let t57021 = t9035 * t8991;
    (t57011, t57013, t57015, t57017, t57019, t57021)
}
