//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2886/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886(t17271: f64, t2815: f64, t896: f64, t17210: f64, t2807: f64, t13615: f64, t4362: f64, t17215: f64, t17218: f64, t17255: f64, t699: f64, t136: f64, t59730: f64, t908: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60263 = t2815 * t17271 * t896;
    let t60265 = t17210 * t2807;
    let t60267 = t4362 * t13615;
    let t60269 = t17215 * t2807;
    let t60271 = t17218 * t2807;
    let t60274 = t699 * t17255;
    let t60277 = t136 * t908 * t59730;
    (t60263, t60265, t60267, t60269, t60271, t60274, t60277)
}
