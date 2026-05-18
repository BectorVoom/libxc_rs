//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1080/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1080<F: Float>(t338: F, t4053: F, t892: F, t1176: F, t2298: F, t367: F, t1178: F, t2402: F, t371: F, t4052: F, t810: F, t2376: F, t2409: F) -> (F, F, F, F, F) {
    let t13826 = t338 * t892 * t4053;
    let t13830 = t1176 * t367 * t2298;
    let t13832 = t371 * t1178 * t2402;
    let t13833 = t13830 * t13832;
    let t13835 = t4052 * t810;
    let t13837 = t2409 * t2376 * t13835;
    (t13826, t13832, t13833, t13835, t13837)
}
