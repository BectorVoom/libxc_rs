//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1057/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1057(t1100: f64, t4756: f64, t1661: f64, t3287: f64, t1102: f64, t1107: f64, t1667: f64, t699: f64, t3297: f64, t4724: f64, t136: f64, t1113: f64, t4729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    let t4772 = t3297 * t4724;
    let t4773 = t136 * t4772;
    let t4775 = t1113 * t4729;
    (t4757, t4764, t4765, t4767, t4770, t4772, t4773, t4775)
}
