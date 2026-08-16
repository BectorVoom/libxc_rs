//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2772/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772(t22026: f64, t46929: f64, t808: f64, t22135: f64, t9744: f64, t1413: f64, t21969: f64, t547: f64, t807: f64, t221: f64, t22274: f64, t3978: f64, t46716: f64) -> (f64, f64, f64, f64) {
    let t74362 = t46929 * t808 * t22026;
    let t74364 = t9744 * t22135;
    let t74402 = t807 * t547 * t1413 * t21969;
    let t74419 = t221 * t22274;
    let t74421 = t3978 * t46716 * t74419;
    (t74362, t74364, t74402, t74421)
}
