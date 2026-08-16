//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 686/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk686(t960: f64, t966: f64, t1: f64, t875: f64, t350: f64, t311: f64, t6194: f64, t5: f64, t830: f64, t2577: f64, t869: f64, t818: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7519 = t960 * t966;
    let t7520 = t875 * t1;
    let t7521 = t7520 * t350;
    let t7522 = t7519 * t7521;
    let t7547 = t311 * t6194;
    let t7549 = t830 * t5;
    let t7553 = t869 * t2577;
    let t7556 = t818 * t959;
    (t7521, t7522, t7547, t7549, t7553, t7556)
}
