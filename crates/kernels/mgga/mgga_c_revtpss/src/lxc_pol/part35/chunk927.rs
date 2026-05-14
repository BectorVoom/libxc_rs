//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 927/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk927<F: Float>(t2013: F, t25081: F, t2411: F, t30: F, t1946: F, t2684: F, t7043: F, t820: F, t843: F, t240: F, t7036: F, t2670: F, t7033: F, t2482: F, t27: F, t1941: F, t243: F) -> (F, F, F, F, F, F, F, F) {
    let t25082 = t2013 * t25081;
    let t25207 = t2411 * t30;
    let t25219 = t1946 * t2684;
    let t25222 = t820 * t7043 * t843;
    let t25227 = t7036 * t240;
    let t25231 = t7033 * t2670;
    let t25234 = t2482 * t7043 * t27;
    let t25237 = t1941 * t243;
    (t25082, t25207, t25219, t25222, t25227, t25231, t25234, t25237)
}
