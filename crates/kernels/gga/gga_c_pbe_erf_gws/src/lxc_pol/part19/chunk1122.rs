//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1122/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1122<F: Float>(t4116: F, t945: F, t1206: F, t810: F, t353: F, t4386: F, t1205: F, t2416: F) -> (F, F, F, F) {
    let t14161 = t4116 * t945;
    let t14180 = t1206 * t810;
    let t14181 = t353 * t14180;
    let t14182 = t4386 * t14181;
    let t14185 = t2416 * t1205;
    (t14161, t14180, t14182, t14185)
}
