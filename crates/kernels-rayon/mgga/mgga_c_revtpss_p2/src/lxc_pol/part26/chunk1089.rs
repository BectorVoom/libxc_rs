//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1089/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1089(t7384: f64, t887: f64, t689: f64, t7399: f64, t786: f64, t789: f64, t231: f64, t7398: f64, t836: f64, t7076: f64, t2061: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26560 = t7384 * t887;
    let t26561 = t689 * t26560;
    let t26563 = t786 * t7399;
    let t26564 = t26563 * t789;
    let t26567 = t7398 * t836 * t231;
    let t26568 = t7076 * t26567;
    let t26573 = t7076 * t2061 * t2645 * t231;
    (t26560, t26561, t26563, t26564, t26568, t26573)
}
