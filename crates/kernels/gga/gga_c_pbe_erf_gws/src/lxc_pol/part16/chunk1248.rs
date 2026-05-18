//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1248/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1248<F: Float>(t14602: F, t51666: F, t3959: F, t9704: F, t3965: F, t9323: F, t13917: F, t14424: F, t9551: F, t14415: F, t51563: F, t13776: F, t36865: F, t3975: F) -> (F, F, F, F, F, F) {
    let t53597 = t51666 * t14602;
    let t53599 = t3959 * t9704;
    let t53601 = t3965 * t9323;
    let t53623 = t13917 * t14424 * t9551;
    let t53625 = t51563 * t14415;
    let t53631 = t13776 * t3975 * t36865;
    (t53597, t53599, t53601, t53623, t53625, t53631)
}
