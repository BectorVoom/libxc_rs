//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2197/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197(t10277: f64, t387: f64, t1625: f64, t225: f64, t344: f64, t12648: f64, t14165: f64, t1927: f64, t23327: f64, t23329: f64, t23332: f64, t23588: f64, t23594: f64, t23728: f64, t25416: f64, t25423: f64, t25425: f64, t25429: f64, t25431: f64, t25432: f64, t25442: f64, t25815: f64, t4548: f64, t6691: f64, t7553: f64, t82402: f64, t82417: f64, t82502: f64, t83352: f64, t88004: f64, t88016: f64, t88022: f64, t88023: f64) -> f64 {
    let t88035 = t387 * t10277;
    let t88050 = t344 * t1625 * t225;
    let t88054 = -0.54831135561607547884e-2_f64 * t23327 * t88004 * t6691 + 0.54831135561607547884e-2_f64 * t23327 * t82502 * t25815 + 0.73108180748810063846e-2_f64 * t25429 * t82417 * t25431 + 0.29243272299524025538e-1_f64 * t82402 * t25425 - 0.19495514866349350359e-1_f64 * t88016 * t25432 + 0.14621636149762012769e-1_f64 * t82402 * t25416 + 0.8529287754027840782e-2_f64 * t88022 * t23329 * t88023 * t14165 + 0.16449340668482264365e-1_f64 * t1927 * t4548 * t23588 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t25423 * t12648 - 0.21932454224643019154e-1_f64 * t25429 * t23329 * t88035 * t14165 - 0.27415567780803773942e-2_f64 * t23327 * t25442 * t23728 - 0.36554090374405031923e-2_f64 * t25429 * t25442 * t23594 - 0.27415567780803773942e-2_f64 * t23327 * t83352 * t7553 - 0.54831135561607547884e-2_f64 * t23327 * t88050 * t23332;
    t88054
}
