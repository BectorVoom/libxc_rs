//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 977/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk977(t22252: f64, t6028: f64, t6027: f64, t20925: f64, t4293: f64, t4292: f64, t17505: f64, t5916: f64, t17450: f64, t2039: f64, t5913: f64, t21804: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22361 = t6028 * t22252;
    let t22362 = t6027 * t22361;
    let t22364 = t4293 * t20925;
    let t22365 = t4292 * t22364;
    let t22367 = t17505 * t5916;
    let t22369 = t17450 * t2039;
    let t22371 = t17505 * t5913;
    let t22373 = t4261 * t21804;
    (t22361, t22362, t22364, t22365, t22367, t22369, t22371, t22373)
}
