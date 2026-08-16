//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2471/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2471(t1070: f64, t193: f64, t336: f64, t47793: f64, t47795: f64, t47798: f64, t47802: f64, t48679: f64, t48681: f64, t48725: f64, t48727: f64, t48730: f64, t48732: f64, t50648: f64, t50678: f64, t50712: f64, t50744: f64) -> f64 {
    let t50750 = t47793 - t47795 + t47798 + t47802 + t193 * t336 * (t50648 + t50678 + t50712 + t50744) * t1070 - t48679 - t48681 - t48725 - t48727 - t48730 - t48732;
    t50750
}
