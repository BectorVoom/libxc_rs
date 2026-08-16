//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1964/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1964(t1769: f64, t7627: f64, t7637: f64, t11239: f64, t1276: f64, t3596: f64, t2149: f64, t29157: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29186 = t7627 * t1769;
    let t29187 = t7637 * t29186;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29195 = t29157 * t3153;
    (t29186, t29187, t29192, t29193, t29194, t29195)
}
