//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 711/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk711(t1600: f64, t2958: f64, t1573: f64, t2932: f64, t2938: f64, t152: f64, t3638: f64, t5918: f64, t434: f64, t144: f64, t467: f64, t458: f64) -> (f64, f64, f64, f64, f64) {
    let t8406 = t1600 * t2958;
    let t8408 = t2932 * t1573;
    let t8409 = t8408 * t2938;
    let t8411 = t3638 * t152;
    let t8412 = t8411 * t5918;
    let t8413 = t434 * t8412;
    let t8415 = t467 * t144;
    let t8416 = t8415 * t458;
    (t8406, t8409, t8413, t8415, t8416)
}
