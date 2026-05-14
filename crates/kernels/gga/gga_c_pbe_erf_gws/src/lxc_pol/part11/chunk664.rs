//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 664/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk664<F: Float>(t3360: F, t460: F, t40: F, t75: F, t472: F, t19: F, t3701: F, t796: F, t801: F, t169: F, t301: F, t3373: F, t784: F, t3379: F, t532: F, t159: F, t285: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10016 = t3360 * t460;
    let t10017 = t40 * t10016;
    let t10020 = t3360 * t75;
    let t10021 = t10020 * t472;
    let t10024 = t3701 * t796 * t19;
    let t10025 = t10024 * t801;
    let t10029 = t169 * t784 * t3373 * t301;
    let t10033 = t532 * t3379;
    let t10035 = t10033 * t159 * t285;
    (t10016, t10017, t10020, t10021, t10024, t10025, t10029, t10033, t10035)
}
