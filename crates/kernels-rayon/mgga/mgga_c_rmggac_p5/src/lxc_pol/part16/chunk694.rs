//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 694/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk694(t209: f64, t558: f64, t605: f64, t511: f64, t1971: f64, t1970: f64, t570: f64, t515: f64, t8443: f64, t8451: f64, t1945: f64, t1986: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10022 = t558 * t605 * t209;
    let t10023 = t511 * t10022;
    let t10024 = t1971 * t10023;
    let t10025 = t1970 * t10024;
    let t10028 = t570 * t605 * t209;
    let t10029 = t515 * t10028;
    let t10030 = t1971 * t10029;
    let t10031 = t1970 * t10030;
    let t10033 = t8451 * t8443;
    let t10040 = t1986 * t1945;
    (t10024, t10025, t10030, t10031, t10033, t10040)
}
