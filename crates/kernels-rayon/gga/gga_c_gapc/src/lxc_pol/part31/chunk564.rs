//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 564/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk564(t1058: f64, t3209: f64, t761: f64, t996: f64, t825: f64, t932: f64, t493: f64, t787: f64, t1055: f64, t773: f64, t2206: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3210 = t3209 * t1058;
    let t3212 = t996 * t761;
    let t3213 = t3212 * t1058;
    let t3216 = t932 * t825;
    let t3217 = t996 * t3216;
    let t3218 = t493 * t787;
    let t3219 = t3217 * t3218;
    let t3221 = t1055 * t773;
    let t3222 = t3209 * t3221;
    let t3224 = t277 * t2206;
    (t3210, t3212, t3213, t3216, t3217, t3218, t3219, t3221, t3222, t3224)
}
