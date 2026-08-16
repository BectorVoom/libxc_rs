//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2940/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940(t1882: f64, t3923: f64, t4003: f64, t10022: f64, t2782: f64, t10014: f64, t14242: f64, t10073: f64, t14225: f64, t1892: f64, t5744: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48073 = t1882 * t3923;
    let t48074 = t48073 * t4003;
    let t48076 = t2782 * t10022 * t48074;
    let t48079 = t10014 * t14242;
    let t48081 = t10073 * t14225;
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    (t48073, t48076, t48079, t48081, t48083, t48084)
}
