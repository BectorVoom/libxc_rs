//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2321/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2321(t17307: f64, t480: f64, t1803: f64, t3650: f64, t16708: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12678: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17308 = t17307 * t480;
    let t17311 = t3650 * t1803;
    let t17319 = 0.37037037037037037037e-2_f64 * t16708;
    let t17320 = 0.11111111111111111111e-1_f64 * t16710;
    let t17321 = 0.55555555555555555556e-2_f64 * t16712;
    let t17330 = -t12678 + 0.74074074074074074074e-2_f64 * t12297 + 0.18518518518518518519e-2_f64 * t12299 - 0.55555555555555555556e-2_f64 * t12301 - 0.27777777777777777778e-2_f64 * t12303 + 0.37037037037037037037e-2_f64 * t16706 + t17319 - t17320 - t17321 + 0.92592592592592592592e-2_f64 * t16717 - 0.33333333333333333333e-1_f64 * t16722 - 0.11111111111111111111e-1_f64 * t16727 - 0.55555555555555555555e-2_f64 * t16731 + 0.50000000000000000001e-1_f64 * t16735 + 0.33333333333333333334e-1_f64 * t16740 + 0.16666666666666666667e-1_f64 * t16744 + 0.83333333333333333333e-2_f64 * t16748;
    (t17308, t17311, t17319, t17320, t17321, t17330)
}
