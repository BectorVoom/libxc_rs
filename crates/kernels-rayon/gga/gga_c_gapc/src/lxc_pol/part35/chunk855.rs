//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 855/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk855(t3330: f64, t9418: f64, t3418: f64, t7522: f64, t3421: f64, t1736: f64, t291: f64, t7949: f64, t959: f64, t9695: f64, t3368: f64, t3371: f64) -> (f64, f64, f64, f64, f64) {
    let t9944 = t9418 * t3330;
    let t9946 = t3418 * t7522;
    let t9948 = t3421 * t7522;
    let t9950 = t1736 * t291;
    let t9952 = t9950 * t959 * t7949;
    let t9953 = t9695 * t9952;
    let t9955 = t3371 * t3368;
    (t9944, t9946, t9948, t9953, t9955)
}
