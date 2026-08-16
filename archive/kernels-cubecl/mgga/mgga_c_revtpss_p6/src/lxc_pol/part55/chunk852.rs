//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 852/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk852<F: Float>(t25374: F, t25386: F, t25378: F, t2769: F, t7056: F, t1955: F, t1949: F, t822: F, t1950: F, t867: F, t786: F, t2467: F) -> (F, F, F, F, F, F) {
    let t25387 = t25386 * t25374;
    let t25388 = t25387 * t25378;
    let t25390 = t7056 * t2769;
    let t25391 = t1955 * t25390;
    let t25392 = t822 * t1949;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    let t25400 = t25399 * t2467;
    (t25387, t25388, t25391, t25392, t25399, t25400)
}
