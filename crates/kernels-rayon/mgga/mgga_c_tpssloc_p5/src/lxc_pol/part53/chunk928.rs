//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 928/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk928(t2039: f64, t649: f64, t7056: f64, t89: f64, t88: f64, t1441: f64, t3701: f64, t8807: f64, t1390: f64, t8803: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34682 = t649 * t2039;
    let t34685 = t89 * t7056;
    let t34707 = t88 * t7056;
    let t35233 = t1441 * t2039;
    let t38018 = t8807 * t3701;
    let t38024 = t8803 * t1390;
    let t39054 = t601 * t9238;
    (t34682, t34685, t34707, t35233, t38018, t38024, t39054)
}
