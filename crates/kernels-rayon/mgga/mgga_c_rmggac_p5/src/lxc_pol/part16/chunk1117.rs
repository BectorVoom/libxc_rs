//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1117/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1117(t49048: f64, t49064: f64, t49082: f64, t49095: f64, t49110: f64, t49126: f64, t49143: f64, t49151: f64, t10484: f64, t2211: f64, t235: f64, t2435: f64, t30221: f64, t30283: f64, t36521: f64, t37976: f64, t43783: f64, t43784: f64, t43792: f64, t47405: f64, t47408: f64, t47410: f64, t47414: f64, t47417: f64, t47429: f64, t47432: f64, t504: f64, t515: f64, t623: f64, t6557: f64, t8264: f64, t884: f64, t9487: f64) -> (f64, f64) {
    let t49154 = t49048 + t49064 + t49082 + t49095 + t49110 + t49126 + t49143 + t49151;
    let t49175 = -0.82764499792523576609e-4_f64 * t36521 - 0.85129199786595678799e-5_f64 * t47405 + t43783 - t43784 - 0.11974241701863808564e0_f64 * t47408 - 0.19957069503106347607e-1_f64 * t235 * t515 * t49154 + 0.5987120850931904282e-1_f64 * t47410 + t37976 - 0.55866037359953414211e-4_f64 * t47414 - 0.11974241701863808564e0_f64 * t47417 + t43792 + 0.79828278012425390428e-1_f64 * t30221 * t2435 - 0.19957069503106347607e-1_f64 * t504 * t10484 + 0.212822999466489197e-4_f64 * t47429 - 0.23948483403727617128e0_f64 * t884 * t8264 * t6557 - 0.23948483403727617128e0_f64 * t884 * t2211 * t30283 - 0.39914139006212695214e-1_f64 * t623 * t9487 - 0.30487649791575028312e-3_f64 * t47432;
    (t49154, t49175)
}
