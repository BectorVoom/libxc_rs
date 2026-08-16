//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1063/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1063(t37353: f64, t39778: f64, t85469: f64, t89: f64, t1555: f64, t9025: f64, t356: f64, t519: f64, t85501: f64, t9054: f64, t1974: f64, t85451: f64) -> (f64, f64, f64, f64, f64) {
    let t86950 = t89 * t37353 * t39778 * t85469;
    let t86954 = t89 * t1555 * t9025 * t85469;
    let t86958 = t89 * t356 * t519 * t85501;
    let t86962 = t89 * t356 * t9054 * t85469;
    let t86966 = t89 * t356 * t1974 * t85451;
    (t86950, t86954, t86958, t86962, t86966)
}
