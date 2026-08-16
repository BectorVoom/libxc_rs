//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1251/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1251(t794: f64, t852: f64, t213: f64, t225: f64, t1914: f64, t40772: f64, t1081: f64, t2752: f64, t22573: f64, t6875: f64, t111: f64, t7222: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82312 = t1914 * t40772;
    let t83555 = t2752 * t1081;
    let t83886 = t6875 * t22573;
    let t84033 = t7222 * t111;
    (t82133, t82159, t82312, t83555, t83886, t84033)
}
