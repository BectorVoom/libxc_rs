//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 976/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk976<F: Float>(t2014: F, t25191: F, t7312: F, t7315: F, t2394: F, t30: F, t1962: F, t198: F, t206: F, t2411: F, t14365: F, t605: F, t775: F, t2430: F, t1946: F, t2684: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25193 = 6.0 * t2014 * t25191;
    let t25194 = t7312 * t7315;
    let t25196 = 2.0 * t2014 * t25194;
    let t25198 = t30 * t2394;
    let t25206 = t198 * t206 * t1962;
    let t25207 = t2411 * t30;
    let t25208 = t25207 * t14365;
    let t25211 = t605 * t775;
    let t25215 = t30 * t2430;
    let t25219 = t1946 * t2684;
    (t25193, t25194, t25196, t25198, t25206, t25207, t25208, t25211, t25215, t25219)
}
