//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 841/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841(t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64, t849: f64, t820: f64, t847: f64, t9516: f64, t2645: f64, t2647: f64, t9621: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9988 = t2697 * t2703;
    let t9990 = t9612 * t842;
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t9997 = t847 * t820 * t9516;
    let t10003 = t2645 * t9621 * t2647;
    (t9988, t9990, t9993, t9994, t9997, t10003)
}
