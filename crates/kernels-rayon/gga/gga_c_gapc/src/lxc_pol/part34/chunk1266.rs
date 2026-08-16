//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1266/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1266(t169: f64, t34159: f64, t5486: f64, t619: f64, t11361: f64, t27658: f64, t2993: f64, t11601: f64, t9291: f64, t3691: f64, t8965: f64, t1030: f64, t1971: f64, t9267: f64, t9272: f64) -> (f64, f64, f64, f64, f64) {
    let t35090 = t169 * t5486 * t34159 * t619;
    let t35093 = t2993 * t11361 * t27658;
    let t35095 = t11601 * t9291;
    let t35097 = t3691 * t8965;
    let t35105 = t1030 * t1971 * t9267 * t9272;
    (t35090, t35093, t35095, t35097, t35105)
}
