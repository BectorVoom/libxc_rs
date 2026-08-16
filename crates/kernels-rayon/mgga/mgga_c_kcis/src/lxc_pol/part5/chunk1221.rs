//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1221/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1221(t18645: f64, t18661: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t18828: f64, t18830: f64, t18833: f64, t18835: f64, t18904: f64, t10923: f64, t10924: f64, t13710: f64, t13945: f64, t13949: f64, t18924: f64, t18927: f64, t18930: f64, t18933: f64, t18935: f64, t18937: f64) -> (f64, f64) {
    let t20431 = 0.3529725e1_f64 * t18835 + 0.264729375e1_f64 * t18828 - 0.3529725e1_f64 * t18830 - 0.17648625e1_f64 * t18833 - 0.34431666666666666667e0_f64 * t18674 + 0.103295e1_f64 * t18679 + 0.11477222222222222222e0_f64 * t18645 - 0.34431666666666666667e0_f64 * t18661 + 0.17215833333333333333e0_f64 * t18669 - 0.516475e0_f64 * t18683 + 0.20839e0_f64 * t18904;
    let t20452 = -0.62517e0_f64 * t18924 + 0.83356e0_f64 * t18927 + 0.20839e0_f64 * t18930 - 0.34731666666666666667e-1_f64 * t18933 - t10923 - t10924 - 0.13892666666666666667e0_f64 * t18935 + 0.69463333333333333333e-1_f64 * t18937 - 0.23154444444444444445e0_f64 * t13945 - 0.45908888888888888888e0_f64 * t13710 + 0.27785333333333333334e0_f64 * t13949;
    (t20431, t20452)
}
