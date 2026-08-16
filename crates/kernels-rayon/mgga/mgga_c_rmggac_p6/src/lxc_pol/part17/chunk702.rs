//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 702/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk702(t10014: f64, t7365: f64, t236: f64, t6182: f64, t1971: f64, t1970: f64, t209: f64, t558: f64, t605: f64, t511: f64, t570: f64, t515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10015 = t7365 * t10014;
    let t10016 = 0.85129199786595678796e-5_f64 * t10015;
    let t10017 = t236 * t6182;
    let t10018 = t1971 * t10017;
    let t10019 = t1970 * t10018;
    let t10020 = 0.42564599893297839398e-5_f64 * t10019;
    let t10022 = t558 * t605 * t209;
    let t10023 = t511 * t10022;
    let t10024 = t1971 * t10023;
    let t10025 = t1970 * t10024;
    let t10026 = 0.25538759935978703638e-4_f64 * t10025;
    let t10028 = t570 * t605 * t209;
    let t10029 = t515 * t10028;
    (t10016, t10018, t10020, t10024, t10026, t10029)
}
