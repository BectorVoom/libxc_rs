//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1152/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1152(t28029: f64, t5177: f64, t5078: f64, t7754: f64, t26930: f64, t5099: f64, t5062: f64, t7748: f64, t1200: f64, t4999: f64, t28012: f64, t28014: f64, t28016: f64, t28018: f64, t28020: f64, t28022: f64, t28025: f64, t28027: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28030 = t28029 * t5177;
    let t28032 = t7754 * t5078;
    let t28034 = t26930 * t5099;
    let t28036 = t7748 * t5062;
    let t28038 = t4999 * t1200;
    let t28040 = -t28012 / 6.0_f64 + t28014 / 16.0_f64 - t28016 / 128.0_f64 + t28018 / 24.0_f64 - t28020 / 24.0_f64 + t28022 / 18.0_f64 - t28025 / 288.0_f64 + t28027 / 128.0_f64 - t28030 / 64.0_f64 - t28032 / 72.0_f64 + t28034 / 96.0_f64 - t28036 / 24.0_f64 - t28038 / 96.0_f64;
    (t28030, t28032, t28034, t28036, t28038, t28040)
}
