//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3894/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894(t1357: f64, t22387: f64, t689: f64, t3899: f64, t6896: f64, t1444: f64, t2782: f64, t4075: f64, t556: f64, t6918: f64, t22453: f64, t47530: f64) -> (f64, f64, f64, f64) {
    let t74810 = t689 * t1357 * t22387;
    let t74813 = t689 * t3899 * t6896;
    let t74824 = t2782 * t556 * t4075 * t6918 * t1444;
    let t74826 = t47530 * t22453;
    (t74810, t74813, t74824, t74826)
}
