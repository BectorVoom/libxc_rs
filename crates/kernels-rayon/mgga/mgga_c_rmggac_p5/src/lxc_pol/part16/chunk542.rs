//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 542/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk542(t2021: f64, t7335: f64, t2020: f64, t2165: f64, t2019: f64, t2169: f64, t1311: f64, t20: f64, t2018: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7336 = t7335 * t2021;
    let t7338 = t2020 * t2165;
    let t7339 = t2019 * t7338;
    let t7341 = t2020 * t2169;
    let t7342 = t2019 * t7341;
    let t7344 = t1311 * t20;
    let t7345 = t7344 * t2018;
    (t7336, t7338, t7339, t7341, t7342, t7344, t7345)
}
