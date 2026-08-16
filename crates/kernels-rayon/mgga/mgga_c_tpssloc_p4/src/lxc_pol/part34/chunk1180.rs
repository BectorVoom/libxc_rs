//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1180/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1180(t1530: f64, t5544: f64, t22960: f64, t5527: f64, t28248: f64, t86721: f64, t5660: f64, t25373: f64, t193: f64, t20756: f64, t5397: f64, t21066: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t105758 = t5544 * t1530;
    let t105759 = t22960 * t105758;
    let t105762 = t5527 * t1530;
    let t105763 = t22960 * t105762;
    let t105766 = t86721 * t28248;
    let t105769 = t1530 * t5660;
    let t105770 = t25373 * t105769;
    let t105773 = t193 * t20756;
    let t105780 = t5397 * t1530;
    let t105787 = t25 * t21066;
    (t105758, t105759, t105762, t105763, t105766, t105769, t105770, t105773, t105780, t105787)
}
