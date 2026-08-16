//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 997/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk997(t1174: f64, t11838: f64, t135: f64, t3556: f64, t3493: f64, t3612: f64, t11812: f64, t1243: f64, t10471: f64, t11715: f64, t11712: f64, t11721: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11839 = t1174 * t11838;
    let t11841 = t135 * t3556;
    let t11842 = t1174 * t11841;
    let t11871 = t3612 * t3493;
    let t11877 = t11812 * t1243;
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11883 = t6739 * t11721;
    (t11839, t11842, t11871, t11877, t11881, t11883)
}
