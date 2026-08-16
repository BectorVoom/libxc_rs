//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1162/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1162(t81095: f64, t81102: f64, t81105: f64, t81124: f64, t81131: f64, t89051: f64, t89054: f64, t89058: f64, t89062: f64, t89069: f64, t89073: f64, t89077: f64, t89081: f64, t89085: f64) -> f64 {
    let t89741 = -t89051 / 6.0_f64 + t89054 + 4.0_f64 * t89058 - t89062 / 18.0_f64 + 4.0_f64 / 9.0_f64 * t81095 - 4.0_f64 / 3.0_f64 * t81102 + 2.0_f64 / 27.0_f64 * t81105 - 6.0_f64 * t89069 + 20.0_f64 / 27.0_f64 * t89073 + 4.0_f64 / 3.0_f64 * t89077 + 4.0_f64 / 3.0_f64 * t89081 - 4.0_f64 / 3.0_f64 * t89085 + 2.0_f64 / 9.0_f64 * t81124 + 20.0_f64 / 243.0_f64 * t81131;
    t89741
}
