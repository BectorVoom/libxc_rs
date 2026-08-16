//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 416/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk416(t2580: f64, t3447: f64, t2508: f64, t3431: f64, t739: f64) -> (f64, f64, f64) {
    let t3448 = t2580 * t3447;
    let t3450 = 0.15381052460284448567e-1_f64 * t2508 * t3448;
    let t3451 = t739 * t3431;
    (t3448, t3450, t3451)
}
