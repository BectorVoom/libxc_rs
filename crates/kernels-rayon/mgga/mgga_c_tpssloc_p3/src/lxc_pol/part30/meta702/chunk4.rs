//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2277/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277(t10165: f64, t1052: f64, t1599: f64, t17575: f64, t17635: f64, t17686: f64, t17691: f64, t23327: f64, t23329: f64, t23336: f64, t23581: f64, t25429: f64, t25430: f64, t25743: f64, t25755: f64, t28515: f64, t4557: f64, t4665: f64, t5919: f64, t6687: f64, t6815: f64, t6816: f64, t7553: f64, t88022: f64, t88023: f64, t88812: f64, t88845: f64, t88868: f64, t88932: f64) -> f64 {
    let t99390 = 0.36554090374405031923e-2_f64 * t25429 * t23329 * t25430 * t17635 - 0.27415567780803773942e-2_f64 * t23327 * t23336 * t28515 + t88812 + 0.73108180748810063846e-2_f64 * t25429 * t23329 * t25430 * t17691 + 0.8529287754027840782e-2_f64 * t88022 * t23329 * t88023 * t17686 + 0.27415567780803773942e-2_f64 * t6687 * t23581 * t28515 + 4.0_f64 * t4557 * t25743 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t88932 - 6.0_f64 * t1052 * t10165 * t6815 * t5919 + 0.54831135561607547884e-2_f64 * t6687 * t88868 * t7553 - t17575 * t6816 - t88845 + 4.0_f64 * t25755 * t4665;
    t99390
}
