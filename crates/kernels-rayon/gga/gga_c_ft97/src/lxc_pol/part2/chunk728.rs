//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 728/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk728(t11437: f64, t3194: f64, t1909: f64, t1820: f64, t920: f64, t1910: f64, t3115: f64, t8392: f64, t1755: f64, t1903: f64, t1902: f64, t1922: f64, t452: f64, t942: f64) -> (f64, f64, f64, f64, f64) {
    let t11438 = t3194 * t11437;
    let t11439 = t1909 * t11438;
    let t11442 = t920 * t1820;
    let t11443 = t1910 * t11442;
    let t11444 = t1909 * t11443;
    let t11448 = 2.0_f64 / 27.0_f64 * t8392 * t3115;
    let t11449 = t920 * t1755;
    let t11450 = t1903 * t11449;
    let t11451 = t1902 * t11450;
    let t11455 = t452 * t1922 * t942;
    (t11439, t11444, t11448, t11451, t11455)
}
