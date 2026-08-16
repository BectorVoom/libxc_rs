//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 682/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk682(t184: f64, t9470: f64, t1580: f64, t185: f64, t21: f64, t2236: f64, t2240: f64, t2301: f64, t2306: f64, t2309: f64, t363: f64, t5: f64, t620: f64, t623: f64, t650: f64, t7745: f64, t8614: f64, t8724: f64, t8732: f64, t8739: f64, t8744: f64, t8751: f64, t8754: f64) -> (f64, f64) {
    let t9471 = t9470 * t184;
    let t9478 = 3.0_f64 / 4.0_f64 * t8614 * t650 + t623 * t8724 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t5 * t2236 * t363 + t623 * t8732 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t5 * t620 * t1580 + 3.0_f64 / 4.0_f64 * t623 * t8739 + 3.0_f64 / 4.0_f64 * t2240 * t2306 + 3.0_f64 / 4.0_f64 * t623 * t8744 + 3.0_f64 / 4.0_f64 * t2240 * t2301 + 3.0_f64 / 2.0_f64 * t2240 * t2309 + 3.0_f64 / 4.0_f64 * t623 * t8751 + 3.0_f64 / 4.0_f64 * t623 * t8754 + t5 * t9471 * t21 / 4.0_f64 + t5 * t185 * t7745 / 4.0_f64;
    (t9471, t9478)
}
