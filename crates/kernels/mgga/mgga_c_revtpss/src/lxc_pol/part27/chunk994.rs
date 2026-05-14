//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 994/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk994<F: Float>(t231: F, t836: F, t886: F, t25392: F, t1950: F, t867: F, t786: F, t2467: F, t233: F, t1949: F, t7056: F, t10073: F, t1955: F, t2760: F, t1957: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25394 = t886 * t836 * t231;
    let t25395 = t25392 * t25394;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    let t25400 = t25399 * t2467;
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = 0.24093411633903331839e-3 * t10073 * t25404;
    let t25407 = t1955 * t2760;
    let t25410 = t1957 * t822;
    (t25394, t25395, t25398, t25399, t25400, t25402, t25403, t25404, t25406, t25407, t25410)
}
