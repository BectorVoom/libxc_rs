//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 860/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk860(t28: f64, t1081: f64, t5142: f64, t5145: f64, t584: f64, t157: f64, t5141: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t5149 = piecewise3(t29, 0.0_f64, 4.0_f64 / 9.0_f64 * t5142 * t1081 - 8.0_f64 / 3.0_f64 * t5145 * t584);
    let t5151 = (t5141 + t5149) * t157;
    t5151
}
