//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 766/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk766(t1499: f64, t3079: f64, t1502: f64, t530: f64, t1111: f64, t1446: f64, t2992: f64, t1476: f64, t3058: f64, t1464: f64, t2973: f64, t2916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12119 = t1499 * t3079;
    let t12121 = t530 * t1502;
    let t12122 = t1111 * t12121;
    let t12168 = t1446 * t2992;
    let t12223 = t1476 * t3058;
    let t12238 = t1464 * t2973;
    let t12265 = t1476 * t2916;
    (t12119, t12121, t12122, t12168, t12223, t12238, t12265)
}
