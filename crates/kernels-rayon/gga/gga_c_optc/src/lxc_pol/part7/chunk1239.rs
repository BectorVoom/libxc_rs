//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1239/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1239(t25560: f64, t8207: f64, t7373: f64, t864: f64, t769: f64, t935: f64, t1: f64, t549: f64, t3916: f64, t2270: f64, t7982: f64, t1885: f64, t2670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25561 = t8207 * t25560;
    let t25562 = t864 * t7373;
    let t25564 = t935 * t769;
    let t25565 = t549 * t1;
    let t25566 = t25564 * t25565;
    let t25570 = t3916 * t25560;
    let t25591 = t2270 * t7982;
    let t25595 = t1885 * t2670;
    (t25561, t25562, t25566, t25570, t25591, t25595)
}
