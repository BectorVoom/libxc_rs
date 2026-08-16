//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2302/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2302(t28651: f64, t607: f64, t6800: f64, t1539: f64, t7582: f64, t82655: f64, t23665: f64, t28622: f64, t17635: f64, t18099: f64, t18154: f64, t23327: f64, t23613: f64, t23633: f64, t25429: f64, t25510: f64, t25511: f64, t25654: f64, t25721: f64, t28613: f64, t28671: f64, t6797: f64, t6799: f64, t82534: f64, t82653: f64, t83233: f64, t83239: f64, t83240: f64, t83245: f64, t83246: f64, t89033: f64, t89399: f64) -> f64 {
    let t99998 = t28651 * t6800 * t607;
    let t100008 = t82655 * t1539 * t7582;
    let t100019 = t23665 * t28622;
    let t100025 = -0.54831135561607547884e-2_f64 * t23327 * t25510 * t25511 * t17635 + 0.36554090374405031923e-2_f64 * t25429 * t25510 * t25721 * t17635 - 0.27415567780803773942e-2_f64 * t23327 * t23613 * t28613 + 0.21932454224643019153e-1_f64 * t82534 * t28671 + 0.73108180748810063845e-2_f64 * t83239 * t83240 * t99998 - 0.10966227112321509577e-1_f64 * t23633 * t83233 * t99998 - 0.54831135561607547883e-2_f64 * t89033 * t89399 - 0.54831135561607547884e-2_f64 * t82653 * t100008 + 0.10966227112321509577e-1_f64 * t83245 * t83246 * t28651 * t25654 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t18154 * t6800 + 0.27415567780803773942e-2_f64 * t100019 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t18099 * t6800;
    t100025
}
