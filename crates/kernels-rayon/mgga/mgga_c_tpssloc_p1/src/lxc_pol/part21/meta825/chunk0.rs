//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2900/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2900(t42086: f64, t42087: f64, t59680: f64, t59684: f64, t59688: f64, t59692: f64, t59694: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64, t60238: f64, t60240: f64) -> f64 {
    let t60513 = 0.19931111111111111111e0_f64 * t59680 - 0.29896666666666666667e0_f64 * t59684 + 0.26574814814814814815e0_f64 * t59688 + 0.11958666666666666667e1_f64 * t59692 - 0.13287407407407407408e0_f64 * t59694 - 0.54771111111111111112e-1_f64 * t60223 - 0.27385555555555555556e-1_f64 * t60226 - 0.36514074074074074075e-1_f64 * t60229 - 0.98587999999999999998e0_f64 * t60232 - 0.49293999999999999999e0_f64 * t60235 + t42086 + t42087 + 0.5696775e1_f64 * t60238 - 0.3071625e0_f64 * t60240;
    t60513
}
