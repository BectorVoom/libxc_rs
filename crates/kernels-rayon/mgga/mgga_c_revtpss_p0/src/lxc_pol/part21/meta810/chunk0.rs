//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2959/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2959(t11670: f64, t370: f64, t16094: f64, t11922: f64, t16021: f64, t4899: f64, t3091: f64, t43240: f64, t4787: f64, t1043: f64, t43279: f64, t15785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53884 = t11670 * t370;
    let t53885 = t16094 * t53884;
    let t53898 = t4899 * t11922 * t16021;
    let t53901 = t3091 * t43240 * t4787;
    let t53904 = t43279 * t1043;
    let t53909 = t15785 * t1043;
    (t53884, t53885, t53898, t53901, t53904, t53909)
}
