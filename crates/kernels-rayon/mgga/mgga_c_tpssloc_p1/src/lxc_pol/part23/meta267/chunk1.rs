//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 940/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk940(t28: f64, t12072: f64, t20385: f64, t20390: f64, t5142: f64, t517: f64, t5966: f64, t157: f64, t20384: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t20394 = piecewise3(t29, 0.0_f64, -8.0_f64 / 27.0_f64 * t12072 * t20385 + 4.0_f64 / 3.0_f64 * t5142 * t5966 + 4.0_f64 / 3.0_f64 * t517 * t20390);
    let t20396 = (t20384 + t20394) * t157;
    t20396
}
