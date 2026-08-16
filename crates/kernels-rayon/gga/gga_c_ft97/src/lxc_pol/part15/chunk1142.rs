//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1142/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1142(t446: f64, t89083: f64, t9770: f64, t81095: f64, t81102: f64, t81105: f64, t81124: f64, t81131: f64, t89047: f64, t89051: f64, t89054: f64, t89058: f64, t89062: f64, t89069: f64, t89073: f64, t89077: f64, t89081: f64) -> (f64, f64) {
    let t89085 = t446 * t9770 * t89083;
    let t89089 = -80.0_f64 / 81.0_f64 * t89047 - t89051 + 6.0_f64 * t89054 + 24.0_f64 * t89058 - t89062 / 3.0_f64 + 8.0_f64 / 3.0_f64 * t81095 - 8.0_f64 * t81102 + 4.0_f64 / 9.0_f64 * t81105 - 36.0_f64 * t89069 + 40.0_f64 / 9.0_f64 * t89073 + 8.0_f64 * t89077 + 8.0_f64 * t89081 - 8.0_f64 * t89085 + 4.0_f64 / 3.0_f64 * t81124 + 40.0_f64 / 81.0_f64 * t81131;
    (t89085, t89089)
}
