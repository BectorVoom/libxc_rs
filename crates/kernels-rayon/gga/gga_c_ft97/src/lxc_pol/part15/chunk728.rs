//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 728/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk728(t167: f64, t20023: f64, t9327: f64, t4839: f64, t569: f64, t925: f64, t1053: f64, t4668: f64, t2185: f64, t605: f64, t1017: f64, t4714: f64) -> (f64, f64, f64, f64, f64) {
    let t20702 = t9327 * t167 * t20023;
    let t20706 = t569 * t4839 * t925;
    let t20709 = t4668 * t1053;
    let t20711 = t2185 * t605 * t20709;
    let t20714 = t1017 * t4714;
    (t20702, t20706, t20709, t20711, t20714)
}
