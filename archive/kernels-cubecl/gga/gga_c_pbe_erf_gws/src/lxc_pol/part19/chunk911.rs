//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 911/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk911<F: Float>(t10037: F, t525: F, t2036: F, t3641: F, t3619: F, t5660: F, t142: F, t2900: F, t2031: F, t4561: F, t7906: F, t7907: F) -> (F, F, F, F, F) {
    let t10186 = t525 * t10037;
    let t10189 = t3641 * t2036;
    let t10194 = t5660 * t3619;
    let t10196 = t142 * t2900;
    let t10197 = t2031 * t10196;
    let t10201 = -t7906 + t7907 + t4561;
    (t10186, t10189, t10194, t10197, t10201)
}
