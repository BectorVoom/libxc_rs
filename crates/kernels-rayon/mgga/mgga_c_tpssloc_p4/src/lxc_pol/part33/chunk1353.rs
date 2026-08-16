//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1353/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1353(t20217: f64, t3: f64, t1933: f64, t1937: f64, t21526: f64, t23604: f64, t23678: f64, t25652: f64, t25653: f64, t25658: f64, t28582: f64, t5866: f64, t7578: f64, t83016: f64, t88372: f64, t99692: f64, t99796: f64, t99799: f64, t99802: f64, t99813: f64, t99834: f64) -> f64 {
    let t106348 = t3 * t20217;
    let t106352 = -0.30279567070605293142e-3_f64 * t99692 * t7578 - 0.60559134141210586284e-3_f64 * t99796 - 0.30279567070605293142e-3_f64 * t99799 + 0.60559134141210586284e-3_f64 * t99802 + 0.60559134141210586284e-3_f64 * t99813 - 0.60559134141210586284e-3_f64 * t99834 + t83016 * t21526 / 384.0_f64 + 0.30279567070605293142e-3_f64 * t88372 * t28582 - 0.30279567070605293142e-3_f64 * t25652 * t25658 * t23604 * t5866 + 0.60559134141210586284e-3_f64 * t25652 * t25653 * t23678 * t5866 + 0.10093189023535097714e-3_f64 * t1933 * t106348 * t1937;
    t106352
}
