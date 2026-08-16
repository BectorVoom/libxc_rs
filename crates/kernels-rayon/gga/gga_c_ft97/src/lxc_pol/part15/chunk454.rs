//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 454/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk454(t1943: f64, t920: f64, t1017: f64, t72: f64, t1023: f64, t1526: f64, t1527: f64, t1942: f64, t342: f64, t343: f64, t1022: f64, t1964: f64, t4417: f64) -> (f64, f64, f64, f64, f64) {
    let t4641 = t1943 * t920;
    let t4645 = t72 * t1017;
    let t4649 = t1023 - t1942 - t1526 * t1527 * t4641 / 12.0_f64 - t342 * t343 * t4645 / 4.0_f64;
    let t4650 = t4649 * t1022;
    let t4652 = t1964 * t4417;
    (t4641, t4645, t4649, t4650, t4652)
}
