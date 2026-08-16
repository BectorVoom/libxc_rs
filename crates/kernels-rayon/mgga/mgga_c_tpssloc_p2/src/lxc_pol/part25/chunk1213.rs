//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1213/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1213(t82122: f64, t10104: f64, t10110: f64, t10111: f64, t10112: f64, t2053: f64, t24281: f64, t2718: f64, t2719: f64, t40890: f64, t7087: f64, t7092: f64, t7106: f64, t82113: f64, t82115: f64, t82120: f64, t82126: f64, t855: f64, t865: f64, t9590: f64) -> f64 {
    let t85060 = 0.3244175520728446583e0_f64 * t82122;
    let t85071 = 0.9869604401089358619e-1_f64 * t82113 - 6.0_f64 * t7087 * t10112 + 6.0_f64 * t9590 * t7092 - 0.46058153871750340221e0_f64 * t82115 - 18.0_f64 * t855 * t10110 * t7106 * t2719 + 0.9869604401089358619e-1_f64 * t82120 - t85060 + 24.0_f64 * t855 * t40890 * t2053 * t10111 - t7087 * t10104 + 6.0_f64 * t855 * t2718 * t24281 * t865 - 0.49348022005446793095e-1_f64 * t82126;
    t85071
}
