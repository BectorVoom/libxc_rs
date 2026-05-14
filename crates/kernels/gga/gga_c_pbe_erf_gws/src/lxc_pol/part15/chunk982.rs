//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 982/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk982<F: Float>(t2409: F, t4400: F, t3965: F, t1192: F, t2074: F, t2376: F, t859: F, t940: F) -> (F, F, F, F, F) {
    let t13976 = t2409 * t4400;
    let t13977 = t3965 * t13976;
    let t13979 = t1192 * t2074;
    let t13981 = t2409 * t2376 * t13979;
    let t13984 = t859 * t940;
    (t13976, t13977, t13979, t13981, t13984)
}
