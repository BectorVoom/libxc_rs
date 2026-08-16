//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1217/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1217(t94383: f64, t96221: f64, t213: f64, t26333: f64, t2453: f64, t26264: f64, t9676: f64, t26072: f64, t26271: f64, t26231: f64, t94921: f64, t10073: f64, t1444: f64, t2102: f64, t25929: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96510 = t94383 * t96221;
    let t96512 = t213 * t26333;
    let t96515 = t2453 * t26264;
    let t96516 = t96515 * t9676;
    let t96527 = t26072 * t26271;
    let t96542 = t94921 * t26231;
    let t96546 = t10073 * t25929 * t2102 * t1444;
    (t96510, t96512, t96516, t96527, t96542, t96546)
}
