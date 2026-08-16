//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2402/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402(t48155: f64, t41680: f64, t41713: f64, t47777: f64, t48153: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t49040: f64) -> f64 {
    let t49200 = 0.5519e0_f64 * t48155;
    let t49208 = 0.20128333333333333334e0_f64 * t41680 - 0.60385000000000000002e0_f64 * t41713 + 0.36231e1_f64 * t47777 + 0.198684e1_f64 * t48153 + t49200 - 0.91983333333333333334e-1_f64 * t48157 - 0.66228e0_f64 * t48159 - 0.33114e0_f64 * t48161 - 0.33114000000000000001e0_f64 * t48163 + 0.11038e0_f64 * t48165 + 0.55190000000000000001e-1_f64 * t48167 - 0.1237865625e0_f64 * t49040;
    t49208
}
