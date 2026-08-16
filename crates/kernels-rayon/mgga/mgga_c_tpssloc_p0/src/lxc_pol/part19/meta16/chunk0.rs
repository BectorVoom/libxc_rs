//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 128/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk128(t273: f64, t276: f64, t279: f64, t285: f64) -> (f64, f64, f64) {
    let t287 = 0.379785e1_f64 * t276 + 0.8969e0_f64 * t273 + 0.204775e0_f64 * t279 + 0.123235e0_f64 * t285;
    let t290 = 1.0_f64 + 0.16081979498692535067e2_f64 / t287;
    let t291 = f64::ln(t290);
    (t287, t290, t291)
}
