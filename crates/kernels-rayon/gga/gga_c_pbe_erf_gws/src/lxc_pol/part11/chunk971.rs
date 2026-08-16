//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 971/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk971(t1620: f64, t17508: f64, t3512: f64, t1673: f64, t3488: f64, t16942: f64, t3530: f64, t587: f64, t10325: f64, t586: f64, t1778: f64, t3479: f64) -> (f64, f64, f64, f64, f64) {
    let t30824 = t1620 * t17508 * t3512;
    let t30839 = t3488 * t1673;
    let t30856 = t587 * t16942 * t3530;
    let t30876 = t10325 * t586;
    let t30889 = t3479 * t1778;
    (t30824, t30839, t30856, t30876, t30889)
}
