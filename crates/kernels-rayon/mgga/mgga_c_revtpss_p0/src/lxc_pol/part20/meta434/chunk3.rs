//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1638/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1638(t126: f64, t13099: f64, t12257: f64, t1261: f64, t247: f64, t12879: f64, t3372: f64, t3368: f64, t1222: f64, t12287: f64, t17240: f64, t12881: f64, t3647: f64) -> (f64, f64, f64, f64, f64) {
    let t44895 = t126 * t13099;
    let t44898 = t1261 * t247 * t44895 * t12257;
    let t44902 = t1261 * t247 * t12879 * t3372;
    let t44906 = t1261 * t247 * t12879 * t3368;
    let t44912 = t1222 * t17240 * t12287;
    let t44917 = t3647 * t12881;
    (t44898, t44902, t44906, t44912, t44917)
}
