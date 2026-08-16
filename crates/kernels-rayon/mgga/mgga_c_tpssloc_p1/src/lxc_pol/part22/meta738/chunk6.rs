//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2428/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428(t59759: f64, t59761: f64, t60308: f64, t60310: f64, t60312: f64, t68638: f64, t68640: f64, t68643: f64, t68646: f64, t68649: f64, t68695: f64, t68697: f64) -> f64 {
    let t69156 = 0.94674375e0_f64 * t68638 + 0.94674375e0_f64 * t68640 - 0.17648625e1_f64 * t68643 + 0.31558125e0_f64 * t68646 - 0.104195e0_f64 * t68649 + 0.309885e1_f64 * t59759 - 0.20659e1_f64 * t59761 - 0.41678000000000000001e0_f64 * t60308 + 0.13892666666666666667e0_f64 * t60310 + 0.9261777777777777778e-1_f64 * t60312 + 0.3529725e1_f64 * t68695 + 0.6311625e0_f64 * t68697;
    t69156
}
