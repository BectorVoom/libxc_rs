//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 720/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk720<F: Float>(t25374: F, t25386: F, t25378: F, t2769: F, t7056: F, t1955: F, t1949: F, t822: F, t1950: F, t867: F, t786: F, t2467: F, t233: F, t10073: F, t1957: F, t676: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25387 = t25386 * t25374;
    let t25388 = t25387 * t25378;
    let t25390 = t7056 * t2769;
    let t25391 = t1955 * t25390;
    let t25392 = t822 * t1949;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    let t25400 = t25399 * t2467;
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = 0.24093411633903331839e-3 * t10073 * t25404;
    let t25410 = t1957 * t822;
    let t25411 = t25386 * t25410;
    let t25412 = t676 * t837;
    (t25387, t25388, t25391, t25392, t25399, t25400, t25406, t25410, t25411, t25412)
}
