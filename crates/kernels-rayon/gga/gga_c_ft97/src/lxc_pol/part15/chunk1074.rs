//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1074/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1074(t62287: f64, t62309: f64, t62317: f64, t78001: f64, t78012: f64, t78015: f64, t78027: f64, t87024: f64, t87027: f64, t87030: f64, t87033: f64, t87037: f64, t87042: f64, t87045: f64, t87048: f64) -> f64 {
    let t87144 = -8.0_f64 / 9.0_f64 * t78001 + 4.0_f64 / 9.0_f64 * t78012 - 16.0_f64 / 9.0_f64 * t78015 - 8.0_f64 * t78027 - 8.0_f64 / 9.0_f64 * t62287 - 4.0_f64 * t87024 - 4.0_f64 * t87027 - 8.0_f64 / 3.0_f64 * t87030 - 16.0_f64 / 3.0_f64 * t87033 - t87037 - 16.0_f64 / 27.0_f64 * t62309 + 16.0_f64 / 9.0_f64 * t62317 + 8.0_f64 * t87042 + 4.0_f64 / 3.0_f64 * t87045 + 8.0_f64 * t87048;
    t87144
}
