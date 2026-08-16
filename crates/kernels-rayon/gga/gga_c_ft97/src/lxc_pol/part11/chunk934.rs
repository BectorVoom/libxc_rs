//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 934/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk934(t2304: f64, t5: f64, t9470: f64, t2253: f64, t8626: f64, t8650: f64, t8662: f64, t8636: f64, t179: f64, t37406: f64, t3628: f64, t634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39390 = t2304 * t2304;
    let t39396 = t5 * t9470;
    let t39402 = t2253 * t8626;
    let t39404 = t2253 * t8650;
    let t39413 = t2253 * t8662;
    let t39415 = t2253 * t8636;
    let t39417 = t179 * t37406;
    let t39422 = t3628 * t634;
    (t39390, t39396, t39402, t39404, t39413, t39415, t39417, t39422)
}
