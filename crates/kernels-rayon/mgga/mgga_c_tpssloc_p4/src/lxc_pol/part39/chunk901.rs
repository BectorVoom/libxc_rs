//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 901/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk901(t109: f64, t104: f64, t50: f64, t656: f64, t1449: f64, t8184: f64, t64: f64, t8128: f64, t8137: f64, t8179: f64, t8262: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t8266 = t656 * t50 * t104;
    let t8269 = t8184 * t1449;
    let t8273 = piecewise3(t110, 0.0_f64, t8179 + t8128 * t8262 / 4.0_f64 + 5.0_f64 / 24.0_f64 * t64 * t8266 - 5.0_f64 / 24.0_f64 * t8137 * t8269);
    (t8266, t8269, t8273)
}
