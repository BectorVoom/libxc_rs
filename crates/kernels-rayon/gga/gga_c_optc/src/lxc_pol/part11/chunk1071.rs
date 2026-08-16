//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1071/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1071(t1111: f64, t1502: f64, t1781: f64, t140: f64, t1514: f64, t7369: f64, t464: f64, t3183: f64, t3101: f64, t1446: f64, t8685: f64, t8581: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34350 = t1111 * t1781 * t1502;
    let t34386 = t1514 * t7369 * t140;
    let t34387 = t464 * t34386;
    let t34390 = t3183 * t34386;
    let t34393 = t3101 * t34386;
    let t34422 = t1446 * t8685;
    let t34434 = t1446 * t8581;
    (t34350, t34387, t34390, t34393, t34422, t34434)
}
