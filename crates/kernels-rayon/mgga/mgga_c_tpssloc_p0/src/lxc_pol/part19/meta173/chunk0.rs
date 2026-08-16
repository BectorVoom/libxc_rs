//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 807/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk807(t2617: f64, t2638: f64, t831: f64, t2639: f64, t2681: f64, t184: f64, t2250: f64, t607: f64, t4194: f64, t116: f64, t126: f64, t136: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9674 = t2617 * t2638;
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9681 = t184 * t2250;
    let t9682 = t9681 * t607;
    let t9684 = 36.0_f64 * t4194 * t9682;
    let t9688 = 1.0_f64 / t126 / t136 * t116 / 4.0_f64;
    (t9674, t9675, t9679, t9681, t9682, t9684, t9688)
}
