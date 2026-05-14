//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1049/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1049<F: Float>(t13793: F, t50943: F, t13803: F, t13808: F, t1192: F, t20154: F, t2376: F, t810: F, t814: F, t13775: F, t13807: F, t13777: F, t371: F, t3970: F, t932: F) -> (F, F, F, F, F, F) {
    let t50970 = t50943 * t13793;
    let t50972 = t13808 * t13803;
    let t50977 = t20154 * t2376 * t1192 * t814 * t810;
    let t50994 = t13807 * t13775;
    let t50995 = t50994 * t13777;
    let t50998 = t3970 * t932 * t371;
    (t50970, t50972, t50977, t50994, t50995, t50998)
}
