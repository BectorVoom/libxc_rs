//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 555/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk555(t1649: f64, t3672: f64, t172: f64, t1787: f64, t763: f64, t67: f64, t758: f64, t193: f64, t533: f64, t1845: f64, t3701: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5142 = t3672 * t1649;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5160 = t193 * t533;
    let t5161 = t1845 * t3701;
    let t5168 = t1787 * t750;
    (t5142, t5154, t5155, t5157, t5158, t5160, t5161, t5168)
}
