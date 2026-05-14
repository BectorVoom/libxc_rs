//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1064/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1064<F: Float>(t10073: F, t1958: F, t25390: F, t886: F, t10665: F, t1949: F, t1955: F, t25308: F, t2769: F, t7049: F, t786: F, t867: F, t2467: F, t2772: F, t689: F, t7014: F) -> (F, F, F, F, F) {
    let t92905 = t10073 * t25390 * t1958 * t886;
    let t92907 = t1949 * t10665;
    let t92917 = t1955 * t25308 * t2769;
    let t92921 = t786 * t7049 * t867;
    let t92922 = t92921 * t2467;
    let t92925 = t689 * t7014 * t2772;
    (t92905, t92907, t92917, t92922, t92925)
}
