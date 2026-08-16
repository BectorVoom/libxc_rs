//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 251/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk251(t284: f64, t712: f64, t293: f64, t711: f64, t291: f64, t279: f64, t481: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1683 = t284 * t284;
    let t1685 = 1.0_f64 / t1683 / t284;
    let t1687 = t1685 * pi * t712;
    let t1691 = 1.0_f64 / t711 / t293;
    let t1692 = t291 * t1691;
    let t1841 = t481 * t729 * t279;
    (t1683, t1685, t1687, t1691, t1692, t1841)
}
