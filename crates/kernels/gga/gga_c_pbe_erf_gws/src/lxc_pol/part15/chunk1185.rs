//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1185/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1185<F: Float>(t14106: F, t2376: F, t829: F, t830: F, t13793: F, t50943: F, t13803: F, t13808: F, t1192: F, t20154: F, t810: F, t814: F) -> (F, F, F, F) {
    let t50965 = t2376 * t14106;
    let t50967 = t829 * t830 * t50965;
    let t50970 = t50943 * t13793;
    let t50972 = t13808 * t13803;
    let t50977 = t20154 * t2376 * t1192 * t814 * t810;
    (t50967, t50970, t50972, t50977)
}
