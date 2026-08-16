//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 712/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk712<F: Float>(t1178: F, t845: F, t371: F, t3983: F, t1176: F, t367: F, t903: F) -> (F, F, F) {
    let t3984 = t1178 * t845;
    let t3985 = t371 * t3984;
    let t3986 = t3983 * t3985;
    let t3989 = t1176 * t367 * t903;
    (t3985, t3986, t3989)
}
