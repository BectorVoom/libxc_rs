//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2302/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2302<F: Float>(t28651: F, t607: F, t6800: F, t1539: F, t7582: F, t82655: F, t23665: F, t28622: F, t17635: F, t18099: F, t18154: F, t23327: F, t23613: F, t23633: F, t25429: F, t25510: F, t25511: F, t25654: F, t25721: F, t28613: F, t28671: F, t6797: F, t6799: F, t82534: F, t82653: F, t83233: F, t83239: F, t83240: F, t83245: F, t83246: F, t89033: F, t89399: F) -> F {
    let t99998 = t28651 * t6800 * t607;
    let t100008 = t82655 * t1539 * t7582;
    let t100019 = t23665 * t28622;
    let t100025 = -F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25510 * t25511 * t17635 + F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25510 * t25721 * t17635 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23613 * t28613 + F::cast_from(0.21932454224643019153e-1_f64) * t82534 * t28671 + F::cast_from(0.73108180748810063845e-2_f64) * t83239 * t83240 * t99998 - F::cast_from(0.10966227112321509577e-1_f64) * t23633 * t83233 * t99998 - F::cast_from(0.54831135561607547883e-2_f64) * t89033 * t89399 - F::cast_from(0.54831135561607547884e-2_f64) * t82653 * t100008 + F::cast_from(0.10966227112321509577e-1_f64) * t83245 * t83246 * t28651 * t25654 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t18154 * t6800 + F::cast_from(0.27415567780803773942e-2_f64) * t100019 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t18099 * t6800;
    t100025
}
