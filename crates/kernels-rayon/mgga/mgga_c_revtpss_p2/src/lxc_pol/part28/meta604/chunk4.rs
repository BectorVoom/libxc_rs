//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2088/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2088(t1444: f64, t25921: f64, t25930: f64, t25931: f64, t27903: f64, t27960: f64, t28003: f64, t5774: f64, t7274: f64, t7295: f64, t7296: f64, t94405: f64, t94409: f64, t94411: f64, t94580: f64, t94584: f64, t94591: f64, t97719: f64, t97734: f64, t97737: f64, t97742: f64) -> f64 {
    let t97752 = t97719 + 0.17347256376410398924e1_f64 * t25921 * t28003 - 0.72280234901709995518e-2_f64 * t94405 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t27960 * t1444 - t94409 + 0.17347256376410398924e1_f64 * t25921 * t27903 + 0.9757440539382783019e-2_f64 * t94411 - t97734 + 0.13009920719177044025e-2_f64 * t94580 - 0.17347256376410398924e1_f64 * t25930 * t25931 * t97737 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t97742 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t7274 * t5774 - 0.54878743191129263322e-2_f64 * t94584 + 0.91399340044406952588e-2_f64 * t94591;
    t97752
}
