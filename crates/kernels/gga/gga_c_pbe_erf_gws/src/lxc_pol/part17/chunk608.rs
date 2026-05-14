//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 608/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk608<F: Float>(t1161: F, t2376: F, t830: F, t829: F, t1105: F, t831: F, t2370: F, t1114: F, t2358: F, t810: F) -> (F, F, F, F, F, F) {
    let t3045 = t2376 * t1161;
    let t3046 = t830 * t3045;
    let t3047 = t829 * t3046;
    let t3050 = t831 * t1105;
    let t3051 = t830 * t3050;
    let t3052 = t2370 * t3051;
    let t3055 = t1114 * t2358;
    let t3060 = t1161 * t810;
    (t3045, t3047, t3050, t3052, t3055, t3060)
}
