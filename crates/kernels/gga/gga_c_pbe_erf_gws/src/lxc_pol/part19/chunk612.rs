//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 612/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk612<F: Float>(t3220: F, t3223: F, t1123: F, t2257: F, t2255: F, t2171: F, t2345: F, t3219: F, t253: F, t903: F) -> (F, F, F, F, F) {
    let t3224 = t3220 * t3223;
    let t3227 = t1123 * t2257;
    let t3228 = t2255 * t3227;
    let t3232 = t2345 * t3219 * t2171;
    let t3235 = t903 * t253;
    (t3224, t3227, t3228, t3232, t3235)
}
