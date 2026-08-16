//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 940/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk940(t28: f64, t1302: f64, t2: f64, t1081: f64, t5178: f64, t584: f64, t5177: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t5181 = t1302 * t2;
    let t5185 = piecewise3(t29, 0.0_f64, -2.0_f64 / 9.0_f64 * t5178 * t1081 - 4.0_f64 / 3.0_f64 * t5181 * t584);
    let t5187 = t5177 / 2.0_f64 + t5185 / 2.0_f64;
    t5187
}
