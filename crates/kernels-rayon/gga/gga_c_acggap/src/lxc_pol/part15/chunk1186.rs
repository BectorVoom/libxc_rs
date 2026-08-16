//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1186/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1186(t2001: f64, t5878: f64, t1988: f64, t9538: f64, t1095: f64, t1426: f64, t38922: f64, t598: f64, t13287: f64, t2302: f64, t31195: f64, t8901: f64) -> (f64, f64, f64, f64) {
    let t40529 = t2001 * t5878;
    let t40533 = t1988 * t9538;
    let t40537 = t598 * t1426 * t1095 * t38922;
    let t40542 = t31195 * t13287 * t2302 * t8901;
    (t40529, t40533, t40537, t40542)
}
