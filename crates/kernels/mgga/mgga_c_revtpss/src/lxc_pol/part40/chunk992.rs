//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 992/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk992<F: Float>(t2645: F, t2723: F, t10115: F, t253: F, t233: F, t2760: F, t869: F, t689: F, t2777: F, t2789: F, t2439: F, t2435: F, t2790: F, t2778: F, t9303: F, t871: F, t9292: F) -> (F, F, F, F, F, F, F) {
    let t10943 = t2723 * t2645;
    let t10948 = 0.11044544084478153697e-3 * t10115 * t253;
    let t10959 = t233 * t2760;
    let t10960 = t869 * t10959;
    let t10961 = t689 * t10960;
    let t10963 = t2777 * t2789;
    let t10964 = t2439 * t10963;
    let t10966 = t2435 * t2790;
    let t10969 = 0.26019841438354088051e-2 * t9303 * t2778;
    let t10971 = 0.17073386770573548589e-1 * t9292 * t871;
    (t10943, t10948, t10961, t10964, t10966, t10969, t10971)
}
