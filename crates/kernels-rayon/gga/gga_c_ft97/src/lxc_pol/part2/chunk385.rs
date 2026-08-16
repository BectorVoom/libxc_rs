//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 385/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk385(t2080: f64, t515: f64, t152: f64, t153: f64, t590: f64, t91: f64, t151: f64, t1771: f64, t1775: f64, t583: f64, t458: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2081 = t515 * t2080;
    let t2086 = 1.0_f64 / t153 / t152;
    let t2087 = t590 * t590;
    let t2089 = t91 * t2086 * t2087;
    let t2092 = 4.0_f64 / 9.0_f64 * t1771 * t151;
    let t2093 = t1775 * t583;
    let t2095 = t458 * t588;
    (t2081, t2086, t2087, t2089, t2092, t2093, t2095)
}
