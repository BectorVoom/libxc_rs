//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 775/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk775(t132: f64, t26078: f64, t36: f64, t4787: f64, t638: f64, t71: f64, t2184: f64, t465: f64, t7472: f64, t7335: f64, t7341: f64, t20: f64, t2018: f64, t2021: f64, t4720: f64) -> (f64, f64, f64, f64, f64) {
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36748 = t7335 * t7341;
    let t36752 = t4720 * t20 * t2018 * t2021;
    (t36700, t36733, t36734, t36748, t36752)
}
