//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1045/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1045(t136: f64, t17292: f64, t13598: f64, t13712: f64, t17149: f64, t17165: f64, t17175: f64, t17189: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64) -> (f64, f64) {
    let t17293 = t136 * t17292;
    let t17295 = -0.26837777777777777779e0_f64 * t13598 + t13712 + 0.16557e0_f64 * t17280 + 0.67094444444444444443e-1_f64 * t17149 - 0.20128333333333333333e0_f64 * t17165 + 0.10064166666666666667e0_f64 * t17175 - 0.301925e0_f64 * t17189 + 0.18396666666666666667e-1_f64 * t17286 - 0.11038e0_f64 * t17288 + 0.5519e-1_f64 * t17290 - 0.82785e-1_f64 * t17293;
    (t17293, t17295)
}
