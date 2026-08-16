//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1791/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1791<F: Float>(t2467: F, t25399: F, t233: F, t867: F, t1949: F, t7056: F, t10073: F, t1955: F, t2760: F, t1957: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t25400 = t25399 * t2467;
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = F::cast_from(0.24093411633903331839e-3_f64) * t10073 * t25404;
    let t25407 = t1955 * t2760;
    let t25410 = t1957 * t822;
    (t25400, t25402, t25403, t25404, t25406, t25407, t25410)
}
