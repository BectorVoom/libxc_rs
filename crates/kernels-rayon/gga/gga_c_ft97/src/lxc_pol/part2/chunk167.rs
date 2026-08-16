//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 167/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk167(t10: f64, t144: f64, t351: f64, t143: f64, t358: f64, t363: f64, t356: f64, t89: f64, t142: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t517 = t10 * t351 * t144;
    let t518 = t517 / 18.0_f64;
    let t519 = t143 * t358;
    let t520 = t519 * t363;
    let t522 = t89 * t356 * t520;
    let t524 = t142 * t142;
    let t525 = 1.0_f64 / t524;
    (t517, t518, t519, t520, t522, t524, t525)
}
