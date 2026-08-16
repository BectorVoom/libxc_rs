//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1179/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1179(t2264: f64, t3813: f64, t123: f64, t1891: f64, t7492: f64, t2263: f64, t2672: f64, t1885: f64, t2274: f64, t875: f64, t896: f64, t10925: f64, t770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24459 = t3813 * t2264;
    let t24464 = t7492 * t1891 * t123;
    let t24468 = t2672 * t2263;
    let t24469 = t1885 * t123;
    let t24470 = t24468 * t24469;
    let t24474 = t3813 * t2274;
    let t24478 = t896 * t875;
    let t24480 = t770 * t10925;
    (t24459, t24464, t24468, t24469, t24470, t24474, t24478, t24480)
}
