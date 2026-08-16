//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 769/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk769(t1053: f64, t1523: f64, t1061: f64, t1531: f64, t2836: f64, t2893: f64, t2937: f64, t2944: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64, t4072: f64, t4080: f64, t4088: f64, t4090: f64, t4093: f64, t4096: f64, t4099: f64, t4102: f64) -> (f64, f64, f64) {
    let t4120 = t1523 * t1053;
    let t4125 = t1531 * t1061;
    let t4142 = -0.17648625e1_f64 * t4072 + 0.3529725e1_f64 * t4080 + t2937 - 0.17215833333333333333e0_f64 * t2836 - 0.17215833333333333333e0_f64 * t4044 - 0.34431666666666666667e0_f64 * t4049 + 0.103295e1_f64 * t4054 + 0.516475e0_f64 * t4058 + 0.31558125e0_f64 * t4088 + 0.6311625e0_f64 * t4090 + t2944 - 0.69463333333333333333e-1_f64 * t2893 - 0.69463333333333333333e-1_f64 * t4093 - 0.34731666666666666667e-1_f64 * t4096 + 0.20839e0_f64 * t4099 + 0.104195e0_f64 * t4102;
    (t4120, t4125, t4142)
}
