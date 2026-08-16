//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1105/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1105<F: Float>(t4018: F, t9270: F, t4055: F, t840: F, t4013: F, t1192: F, t2416: F) -> (F, F, F, F) {
    let t13875 = t9270 * t4018;
    let t13884 = t840 * t4055;
    let t13886 = t840 * t4013;
    let t13888 = t2416 * t1192;
    (t13875, t13884, t13886, t13888)
}
