//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 857/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk857(t182: f64, t6320: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t3918: f64, t6299: f64, t6300: f64, t6301: f64, t6304: f64) -> (f64, f64) {
    let t6322 = 0.19751673498613801407e-1_f64 * t6320 * t182;
    let t6323 = 6.0_f64 * t3918 * t6301 + t2408 + t2417 - t2423 - t2426 + t3686 + t3688 - t3690 - t3695 + t3813 - t6299 - t6300 + t6304 + t6322;
    (t6322, t6323)
}
