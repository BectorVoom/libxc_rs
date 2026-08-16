//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1200/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1200(t1988: f64, t9538: f64, t1095: f64, t1426: f64, t38922: f64, t598: f64, t13287: f64, t2302: f64, t31195: f64, t8901: f64, t1782: f64, t1992: f64, t2095: f64) -> (f64, f64, f64, f64) {
    let t40533 = t1988 * t9538;
    let t40537 = t598 * t1426 * t1095 * t38922;
    let t40542 = t31195 * t13287 * t2302 * t8901;
    let t40546 = t2095 * t1992 * t1782;
    (t40533, t40537, t40542, t40546)
}
