//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1146/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1146(t1385: f64, t8085: f64, t198: f64, t206: f64, t8019: f64, t136: f64, t2457: f64, t8006: f64, t93377: f64, t2435: f64, t8011: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102656 = t1385 * t8085;
    let t102888 = t198 * t206 * t8019;
    let t102980 = t8006 * t136 * t2457;
    let t102981 = t93377 * t102980;
    let t102993 = t8011 * t2435;
    let t102994 = t25431 * t102993;
    (t102656, t102888, t102980, t102981, t102993, t102994)
}
