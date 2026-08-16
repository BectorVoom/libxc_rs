//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 564/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk564(t4579: f64, t85: f64, t3387: f64, t3389: f64, t3391: f64, t185: f64, t4595: f64, t108: f64, t176: f64) -> (f64, f64, f64, f64, f64) {
    let t4604 = 0.19751789702565206229e-1_f64 * t4579 * t85;
    let t4606 = 0.11696446794910408142e1_f64 * t3387;
    let t4607 = 8.0_f64 * t3389;
    let t4608 = 8.0_f64 * t3391;
    let t4609 = t185 * t4595;
    let t4611 = t176 * t4609 * t108;
    (t4604, t4606, t4607, t4608, t4611)
}
