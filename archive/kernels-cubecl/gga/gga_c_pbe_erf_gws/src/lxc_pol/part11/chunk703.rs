//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 703/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk703<F: Float>(t3342: F, t4757: F, t3351: F, t4767: F, t3360: F, t460: F, t40: F, t75: F, t472: F, t19: F, t3701: F, t796: F) -> (F, F, F, F, F, F, F) {
    let t9981 = t4757 * t3342;
    let t9993 = t4767 * t3351;
    let t10016 = t3360 * t460;
    let t10017 = t40 * t10016;
    let t10020 = t3360 * t75;
    let t10021 = t10020 * t472;
    let t10024 = t3701 * t796 * t19;
    (t9981, t9993, t10016, t10017, t10020, t10021, t10024)
}
