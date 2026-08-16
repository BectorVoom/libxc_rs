//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 958/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk958(t1286: f64, t1637: f64, t7217: f64, t497: f64, t7165: f64, t7211: f64, t32053: f64, t92: f64, t32374: f64, t376: f64, t22870: f64, t7162: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t137262 = 4.0_f64 / 27.0_f64 * t1286 * t1637 * t7217;
    let t137298 = t7165 * t497;
    let t137311 = t7211 * t497;
    let t137324 = t32053 * t92;
    let t137350 = t1286 * t376 * t32374;
    let t137353 = 2.0_f64 / 27.0_f64 * t7162 * t22870;
    (t137262, t137298, t137311, t137324, t137350, t137353)
}
