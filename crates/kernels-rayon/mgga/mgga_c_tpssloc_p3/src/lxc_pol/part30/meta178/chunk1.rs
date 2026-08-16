//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 892/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk892(t1100: f64, t4756: f64, t1661: f64, t3287: f64, t1102: f64, t1107: f64, t1667: f64, t699: f64) -> (f64, f64, f64, f64, f64) {
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    (t4757, t4764, t4765, t4767, t4770)
}
