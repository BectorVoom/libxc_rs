//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1031/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1031(t4254: f64, t8457: f64, t1936: f64, t7221: f64, t651: f64, t7003: f64, t8634: f64, t196: f64, t197: f64, t7231: f64, t2035: f64, t6985: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32309 = t4254 * t8457;
    let t32311 = t7221 * t1936;
    let t32312 = t651 * t32311;
    let t32320 = 4.0_f64 * t8634 * t7003;
    let t32322 = t7231 * t196 * t197;
    let t32323 = t32322 * t2035;
    let t32325 = t6985 * t7003;
    (t32309, t32311, t32312, t32320, t32322, t32323, t32325)
}
