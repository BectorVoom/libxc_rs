//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1150/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1150(t12207: f64, t9823: f64, t41528: f64, t41532: f64, t41534: f64, t41544: f64, t44170: f64, t44174: f64, t44178: f64, t44179: f64, t44180: f64, t44181: f64, t44185: f64, t44186: f64) -> f64 {
    let t47572 = t9823 * t12207;
    let t47574 = 0.38342925953920749677e0_f64 * t41528;
    let t47575 = 0.85206502119823888171e-1_f64 * t41532;
    let t47576 = 0.38342925953920749677e0_f64 * t41534;
    let t47578 = -t44170 - t44174 - t44178 + 0.35750489951850426669e0_f64 * t47572 - t44179 - t44180 + t44181 - t47574 + t47575 - t47576 + t44185 + t44186 - 0.76685851907841499354e0_f64 * t41544;
    t47578
}
