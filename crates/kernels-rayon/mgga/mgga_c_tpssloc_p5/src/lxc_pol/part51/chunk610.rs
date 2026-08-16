//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 610/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk610(t3238: f64, t3274: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t1100: f64, t1661: f64, t3287: f64, t1102: f64, t1107: f64, t1667: f64, t699: f64) -> (f64, f64, f64, f64) {
    let t4756 = t3274 - t3238 / 9.0_f64 - t4721 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4726 + 2.0_f64 / 3.0_f64 * t4731 + t4735 / 3.0_f64;
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    (t4757, t4765, t4767, t4770)
}
