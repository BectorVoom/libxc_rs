//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2293/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2293(t27548: f64, t8020: f64, t29614: f64, t491: f64, t103218: f64, t1653: f64, t17691: f64, t19225: f64, t2155: f64, t24589: f64, t24590: f64, t24601: f64, t27388: f64, t27426: f64, t27433: f64, t27445: f64, t27446: f64, t27549: f64, t27774: f64, t27776: f64, t29690: f64, t5059: f64, t65203: f64, t66822: f64, t7283: f64, t7287: f64, t7288: f64, t7300: f64, t8002: f64, t85674: f64, t94374: f64, t94378: f64, t94395: f64, t94514: f64) -> (f64, f64) {
    let t103223 = t8020 * t27548;
    let t103226 = t29614 * t491;
    let t103258 = -0.54831135561607547884e-2_f64 * t7283 * t27426 * t27388 - 0.26806332941230356743e-1_f64 * t103218 * t7288 + 0.29243272299524025538e-1_f64 * t94395 * t27446 - 0.19495514866349350359e-1_f64 * t103223 * t27776 + 0.27415567780803773942e-2_f64 * t24589 * t103226 * t7287 + 0.73108180748810063846e-2_f64 * t27549 * t24601 * t27774 * t17691 - 0.36554090374405031923e-2_f64 * t27549 * t24590 * t29690 - 0.54831135561607547884e-2_f64 * t24589 * t94514 * t27433 + 0.54831135561607547884e-2_f64 * t24589 * t94374 * t8002 + 0.10966227112321509577e-1_f64 * t24589 * t94514 * t27445 - 0.10966227112321509577e-1_f64 * t24589 * t94378 * t1653 * t5059 - 0.49348022005446793095e-1_f64 * t7283 * t7300 * t85674 * t19225 - 2.0_f64 * t66822 * t2155 - 2.0_f64 * t65203 * t2155;
    (t103223, t103258)
}
