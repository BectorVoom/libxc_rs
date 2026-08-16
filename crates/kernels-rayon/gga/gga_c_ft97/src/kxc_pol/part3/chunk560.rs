//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 560/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk560(t370: f64, t4495: f64, t27: f64, t89: f64, t1545: f64, t3161: f64, t3166: f64, t4420: f64, t4424: f64, t4428: f64, t4434: f64, t4439: f64) -> (f64, f64, f64) {
    let t4496 = t370 * t4495;
    let t4498 = t89 * t27 * t4496;
    let t4500 = t1545 + t3161 + t3166 - t4420 / 27.0_f64 + t4424 / 9.0_f64 + t4428 / 9.0_f64 - t4434 / 18.0_f64 + t4439 / 3.0_f64 - t4498 / 6.0_f64;
    (t4496, t4498, t4500)
}
