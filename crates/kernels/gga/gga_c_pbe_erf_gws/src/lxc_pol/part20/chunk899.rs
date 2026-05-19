//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 899/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk899<F: Float>(t10006: F, t87: F, t40: F, t3360: F, t460: F, t4755: F, t7997: F, t75: F, t472: F, t19: F, t3701: F, t796: F) -> (F, F, F, F, F, F) {
    let t10014 = t10006 * t87;
    let t10015 = t40 * t10014;
    let t10016 = t3360 * t460;
    let t10017 = t40 * t10016;
    let t10018 = F::new(12.0) * t4755;
    let t10019 = F::new(2.0) * t7997;
    let t10020 = t3360 * t75;
    let t10021 = t10020 * t472;
    let t10022 = F::cast_from(0.58482233974552040708e0_f64) * t10021;
    let t10024 = t3701 * t796 * t19;
    (t10015, t10017, t10018, t10019, t10022, t10024)
}
