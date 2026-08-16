//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1185/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1185(t25810: f64, t7553: f64, t5685: f64, t6690: f64, t6689: f64, t1922: f64, t5844: f64, t1052: f64, t1635: f64, t1920: f64, t25450: f64, t25736: f64, t25755: f64, t25778: f64, t28470: f64, t28475: f64, t28481: f64, t28485: f64, t28488: f64, t28492: f64, t28496: f64, t28500: f64, t28505: f64, t388: f64, t4660: f64, t6687: f64, t7600: f64, t7625: f64) -> (f64, f64, f64, f64, f64) {
    let t28510 = t25810 * t7553;
    let t28515 = t6690 * t5685;
    let t28516 = t6689 * t28515;
    let t28519 = t5844 * t1922;
    let t28523 = 0.16449340668482264365e-1_f64 * t6687 * t28470 + 0.82246703342411321825e-2_f64 * t1920 * t28475 + 4.0_f64 * t4660 * t7600 - 0.82246703342411321825e-2_f64 * t6687 * t28481 + 4.0_f64 * t1052 * t28485 + 2.0_f64 * t28488 * t388 + 0.36554090374405031923e-2_f64 * t6687 * t28492 + 0.16449340668482264365e-1_f64 * t6687 * t28496 - 0.54831135561607547884e-2_f64 * t6687 * t28500 - 2.0_f64 * t4660 * t7625 + t28505 * t388 + 0.18277045187202515961e-2_f64 * t25450 - 2.0_f64 * t25778 * t1635 + 0.54831135561607547884e-2_f64 * t6687 * t28510 - 2.0_f64 * t25755 * t1635 + 0.27415567780803773942e-2_f64 * t6687 * t28516 - 0.82246703342411321825e-2_f64 * t6687 * t28519 - 0.54831135561607547884e-2_f64 * t25736;
    (t28510, t28515, t28516, t28519, t28523)
}
