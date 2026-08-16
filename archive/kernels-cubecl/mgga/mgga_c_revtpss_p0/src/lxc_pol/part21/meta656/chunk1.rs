//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2446/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2446<F: Float>(t11144: F, t3252: F, t11852: F, t126: F, t1063: F, t11145: F, t247: F, t11679: F, t11710: F, t3091: F, t11247: F, t11249: F) -> (F, F, F, F) {
    let t42518 = t3252 * t11144;
    let t42534 = t126 * t11852;
    let t42537 = t1063 * t247 * t42534 * t11145;
    let t42546 = t3091 * t11710 * t11679;
    let t42550 = t11247 * t11249;
    (t42518, t42537, t42546, t42550)
}
