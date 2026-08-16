//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 762/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk762(t2798: f64, t2799: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64) -> (f64, f64, f64) {
    let t2800 = t2798 * t2799;
    let t2802 = 4.0_f64 / 9.0_f64 * t2764;
    let t2807 = t2802 + 2.0_f64 / 9.0_f64 * t2766 - 2.0_f64 / 9.0_f64 * t2773 + 2.0_f64 / 3.0_f64 * t2778 - t2782 / 3.0_f64;
    (t2800, t2802, t2807)
}
