//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 512/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk512(t3014: f64, t3033: f64, t981: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t341: f64) -> (f64, f64, f64, f64) {
    let t3034 = t3033 * t3014;
    let t3036 = 0.17315859105681463759e2_f64 * t981 * t3034;
    let t3037 = 0.11111111111111111111e-1_f64 * t2846;
    let t3042 = t3037 + 0.55555555555555555556e-2_f64 * t2848 - 0.55555555555555555555e-2_f64 * t2855 + 0.16666666666666666667e-1_f64 * t2860 - 0.83333333333333333333e-2_f64 * t2864;
    let t3043 = t3042 * t341;
    (t3034, t3036, t3042, t3043)
}
