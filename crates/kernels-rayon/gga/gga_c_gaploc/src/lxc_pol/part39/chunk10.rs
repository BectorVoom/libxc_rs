//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 10/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk10(t22: f64, t21: f64, t5: f64, t11: f64, t14: f64, t17: f64, t13: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23 = 1.0_f64 / t22;
    let t25 = t21 * t5 * t23;
    let t27 = 0.379785e1_f64 * t14 + 0.8969e0_f64 * t11 + 0.204775e0_f64 * t17 + 0.123235e0_f64 * t25;
    let t30 = 1.0_f64 + 0.16081824322151104822e2_f64 / t27;
    let t31 = f64::ln(t30);
    let t33 = 0.62182e-1_f64 * t13 * t31;
    (t23, t25, t27, t30, t31, t33)
}
