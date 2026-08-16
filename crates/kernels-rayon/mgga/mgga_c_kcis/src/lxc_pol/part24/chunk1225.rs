//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1225/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1225(t28059: f64, t5062: f64, t19937: f64, t7754: f64, t19849: f64, t26930: f64, t19655: f64, t3338: f64, t26938: f64, t29062: f64, t26924: f64, t99945: f64, t99948: f64, t99950: f64, t99952: f64, t99954: f64, t99956: f64, t99958: f64, t99960: f64, t99962: f64, t99964: f64, t99966: f64, t99968: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99970 = t28059 * t5062;
    let t99972 = t7754 * t19937;
    let t99974 = t26930 * t19849;
    let t99977 = t7754 * t3338 * t19655;
    let t99979 = t26938 * t29062;
    let t99981 = t26924 * t29062;
    let t99983 = -t99945 / 48.0_f64 - t99948 / 144.0_f64 - t99950 / 9.0_f64 + t99952 / 128.0_f64 - t99954 / 12.0_f64 - t99956 / 12.0_f64 - t99958 / 12.0_f64 - t99960 / 24.0_f64 + t99962 / 54.0_f64 - t99964 / 24.0_f64 - t99966 / 16.0_f64 - t99968 / 3.0_f64 - t99970 / 12.0_f64 + t99972 / 54.0_f64 + t99974 / 48.0_f64 + t99977 / 24.0_f64 + t99979 / 24.0_f64 - t99981 / 9.0_f64;
    (t99970, t99972, t99974, t99977, t99979, t99981, t99983)
}
