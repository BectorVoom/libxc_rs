//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1983/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1983(t1448: f64, t1907: f64, t28197: f64, t28196: f64, t7316: f64, t7898: f64, t13426: f64, t1936: f64, t18227: f64, t4248: f64, t7002: f64, t27123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28198 = t1907 * t1448;
    let t28199 = t28197 * t28198;
    let t28201 = 2.0_f64 * t28196 * t28199;
    let t28202 = t7898 * t7316;
    let t28212 = 2.0_f64 * t13426 * t1936;
    let t28214 = 2.0_f64 * t18227 * t1936;
    let t28216 = 2.0_f64 * t4248 * t7002;
    let t28218 = 2.0_f64 * t27123 * t1936;
    (t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218)
}
