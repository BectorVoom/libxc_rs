//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1212/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1212<F: Float>(t4083: F, t6745: F, t4110: F, t6781: F, t829: F, t830: F, t27047: F, t3067: F, t4097: F, t814: F, t20154: F, t2376: F, t4088: F) -> (F, F, F, F) {
    let t52270 = t6745 * t4083;
    let t52274 = t6781 * t4110;
    let t52276 = t829 * t830 * t52274;
    let t52294 = t27047 * t3067 * t4097 * t814;
    let t52299 = t20154 * t2376 * t4088 * t814;
    (t52270, t52276, t52294, t52299)
}
