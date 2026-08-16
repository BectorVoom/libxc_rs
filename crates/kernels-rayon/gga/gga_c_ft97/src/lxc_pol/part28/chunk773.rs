//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 773/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk773(t103: f64, t7165: f64, t379: f64, t8217: f64, t1307: f64, t452: f64, t5750: f64, t1871: f64, t499: f64, t1882: f64, t7222: f64, t7235: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32494 = t103 * t7165;
    let t32495 = t32494 * t379;
    let t32496 = t8217 * t32495;
    let t32500 = t452 * t5750 * t1307;
    let t32504 = t1871 * t499 * t7165;
    let t32508 = 2.0_f64 / 9.0_f64 * t1882 * t7222;
    let t32510 = t1882 * t7235 / 9.0_f64;
    (t32494, t32495, t32496, t32500, t32504, t32508, t32510)
}
