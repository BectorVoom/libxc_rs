//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 916/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk916(t14640: f64, t17034: f64, t1325: f64, t8002: f64, t14525: f64, t14630: f64) -> (f64, f64, f64, f64) {
    let t17212 = t14640 * t17034;
    let t17215 = t8002 * t1325;
    let t17216 = t14525 * t17215;
    let t17219 = t14630 * t1325;
    (t17212, t17215, t17216, t17219)
}
