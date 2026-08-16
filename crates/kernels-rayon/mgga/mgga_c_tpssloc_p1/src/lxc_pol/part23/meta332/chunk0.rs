//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1100/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1100(t39035: f64, t14: f64, t2230: f64, t594: f64, t9223: f64, t22811: f64, t19: f64, t85: f64, t24: f64, t10276: f64, t73: f64, t11152: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39036 = 0.74688e4_f64 * t39035;
    let t39037 = t14 * t2230;
    let t39038 = 0.175056e5_f64 * t39037;
    let t39039 = t594 * t9223;
    let t39040 = 0.1822464e5_f64 * t39039;
    let t39041 = 1.0_f64 / t22811;
    let t39043 = 0.683424e4_f64 * t19 * t39041;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = 1.0_f64 / t73 / t10276;
    let t39114 = 1.0_f64 / t76 / t11152;
    (t39036, t39037, t39038, t39040, t39043, t39063, t39096, t39114)
}
