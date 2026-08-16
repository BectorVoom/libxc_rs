//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 952/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk952(t11619: f64, t919: f64, t3209: f64, t3254: f64, t3739: f64, t1061: f64, t6179: f64, t2440: f64, t3728: f64, t2212: f64, t2268: f64, t3738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11620 = t11619 * t919;
    let t11621 = t3209 * t11620;
    let t11623 = t3254 * t3739;
    let t11625 = t1061 * t6179;
    let t11626 = t3728 * t2440;
    let t11627 = t11625 * t11626;
    let t11629 = t2268 * t2212;
    let t11630 = t3738 * t11629;
    (t11620, t11621, t11623, t11625, t11626, t11627, t11629, t11630)
}
