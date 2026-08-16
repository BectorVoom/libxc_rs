//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 272/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk272(t1045: f64, t143: f64, t160: f64, t1000: f64, t1020: f64, t1041: f64, t607: f64) -> (f64, f64) {
    let t1047 = t143 * t1045 * t160;
    let t1053 = t1041 / 2.0_f64 - t607 - t1000 / 3.0_f64 - t1020;
    (t1047, t1053)
}
