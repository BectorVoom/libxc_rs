//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1144/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1144(t17276: f64, t973: f64, t16857: f64, t2418: f64, t16890: f64, t2367: f64, t999: f64, t17314: f64, t176: f64, t998: f64, t16918: f64, t4038: f64, t8152: f64) -> (f64, f64, f64, f64, f64) {
    let t49417 = t17276 * t973;
    let t49581 = t16857 * t2418;
    let t49707 = t999 * t2367 * t16890;
    let t49754 = t176 * t17314 * t998;
    let t49773 = t4038 * t8152 * t16918;
    (t49417, t49581, t49707, t49754, t49773)
}
