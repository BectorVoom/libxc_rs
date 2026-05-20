//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1393/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1393<F: Float>(t10910: F, t822: F, t10959: F, t2439: F, t2777: F, t686: F, t72: F, t874: F, t10914: F, t2710: F, t9285: F, t10972: F, t2470: F) -> (F, F, F, F, F) {
    let t40927 = t822 * t10910;
    let t40938 = t2439 * t2777 * t10959;
    let t40942 = t874 * t10910 * t72 * t686;
    let t40945 = t2710 * t10914 * t9285;
    let t40948 = t874 * t10972 * t2470;
    (t40927, t40938, t40942, t40945, t40948)
}
