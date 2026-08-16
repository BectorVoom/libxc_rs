//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 853/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk853(t1593: f64, t37481: f64, t7859: f64, t22686: f64, t8003: f64, t1611: f64, t533: f64, t37454: f64, t384: f64, t7977: f64, t7924: f64, t8002: f64) -> (f64, f64, f64, f64, f64) {
    let t37482 = t37481 * t1593;
    let t37483 = t37482 * t7859;
    let t37484 = t22686 * t8003;
    let t37487 = t1611 * t533;
    let t37488 = t37487 * t37454;
    let t37495 = t384 * t7977;
    let t37499 = t8002 * t7924;
    (t37483, t37484, t37488, t37495, t37499)
}
