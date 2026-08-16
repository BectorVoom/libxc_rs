//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1092/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1092(t39033: f64, t587: f64, t598: f64, t14: f64, t2230: f64, t594: f64, t9223: f64, t22811: f64, t19: f64, t604: f64, t9226: f64, t2233: f64, t2239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39034 = 1638.0_f64 * t39033;
    let t39035 = t587 * t598;
    let t39036 = 0.74688e4_f64 * t39035;
    let t39037 = t14 * t2230;
    let t39038 = 0.175056e5_f64 * t39037;
    let t39039 = t594 * t9223;
    let t39040 = 0.1822464e5_f64 * t39039;
    let t39041 = 1.0_f64 / t22811;
    let t39043 = 0.683424e4_f64 * t19 * t39041;
    let t39046 = t9226 * t604;
    let t39049 = t2233 * t2239;
    (t39034, t39035, t39036, t39037, t39038, t39040, t39043, t39046, t39049)
}
