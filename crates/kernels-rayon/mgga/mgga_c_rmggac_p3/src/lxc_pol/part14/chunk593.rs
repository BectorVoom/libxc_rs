//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 593/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk593(t3351: f64, t7738: f64, t511: f64, t798: f64, t3352: f64, t2144: f64, t4905: f64, t1971: f64, t352: f64, t495: f64, t515: f64, t7230: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7739 = t3351 * t7738;
    let t7740 = 0.25538759935978703638e-4_f64 * t7739;
    let t7741 = t511 * t798;
    let t7742 = t3352 * t7741;
    let t7743 = t3351 * t7742;
    let t7744 = 0.76616279807936110914e-4_f64 * t7743;
    let t7745 = t2144 * t4905;
    let t7746 = t1971 * t7745;
    let t7747 = t3351 * t7746;
    let t7748 = 0.25538759935978703638e-4_f64 * t7747;
    let t7750 = t515 * t352 * t495;
    let t7751 = t1971 * t7750;
    let t7752 = t7230 * t7751;
    (t7740, t7742, t7744, t7746, t7748, t7751, t7752)
}
