//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 781/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk781(t1561: f64, t923: f64, t1569: f64, t931: f64, t2766: f64, t2824: f64, t2868: f64, t2875: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t4363: f64, t4371: f64, t4379: f64, t4381: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64) -> (f64, f64, f64) {
    let t4411 = t1561 * t923;
    let t4416 = t1569 * t931;
    let t4433 = -0.17648625e1_f64 * t4363 + 0.3529725e1_f64 * t4371 + t2868 + 0.17215833333333333333e0_f64 * t2766 + 0.17215833333333333333e0_f64 * t4335 - 0.34431666666666666667e0_f64 * t4340 + 0.103295e1_f64 * t4345 - 0.516475e0_f64 * t4349 + 0.31558125e0_f64 * t4379 + 0.6311625e0_f64 * t4381 + t2875 + 0.69463333333333333333e-1_f64 * t2824 + 0.69463333333333333333e-1_f64 * t4384 - 0.34731666666666666667e-1_f64 * t4387 + 0.20839e0_f64 * t4390 - 0.104195e0_f64 * t4393;
    (t4411, t4416, t4433)
}
