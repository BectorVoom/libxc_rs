//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1011/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1011(t1054: f64, t11616: f64, t125: f64, t825: f64, t919: f64, t3209: f64, t3254: f64, t3739: f64, t1061: f64, t6179: f64, t2440: f64, t3728: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11617 = t1054 * t11616;
    let t11619 = t825 * t125;
    let t11620 = t11619 * t919;
    let t11621 = t3209 * t11620;
    let t11623 = t3254 * t3739;
    let t11625 = t1061 * t6179;
    let t11626 = t3728 * t2440;
    (t11617, t11619, t11620, t11621, t11623, t11625, t11626)
}
