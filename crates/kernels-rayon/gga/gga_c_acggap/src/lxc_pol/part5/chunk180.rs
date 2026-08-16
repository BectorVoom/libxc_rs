//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 180/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk180(t336: f64, t337: f64, t495: f64, t346: f64, t345: f64, t344: f64, t359: f64) -> (f64, f64, f64, f64) {
    let t500 = t336 * t337 * t495;
    let t503 = t346 * t495;
    let t504 = t345 * t503;
    let t506 = -t344 - t504 / 4.0_f64 + t359;
    (t500, t503, t504, t506)
}
