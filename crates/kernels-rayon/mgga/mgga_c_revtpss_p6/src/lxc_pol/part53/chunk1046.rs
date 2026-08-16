//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1046/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1046(t2007: f64, t7741: f64, t651: f64, t4248: f64, t8461: f64, t7732: f64, t1843: f64, t8460: f64, t8457: f64, t1936: f64, t7883: f64, t5542: f64, t8595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33574 = t2007 * t7741;
    let t33575 = t651 * t33574;
    let t33577 = t4248 * t8461;
    let t33578 = 2.0_f64 * t33577;
    let t33579 = t7732 * t8461;
    let t33580 = 2.0_f64 * t33579;
    let t33581 = t1843 * t8460;
    let t33582 = t651 * t33581;
    let t33583 = 2.0_f64 * t33582;
    let t33587 = t4248 * t8457;
    let t33589 = t7732 * t8457;
    let t33591 = t7883 * t1936;
    let t33592 = t651 * t33591;
    let t33594 = t8595 * t5542;
    (t33574, t33575, t33578, t33580, t33581, t33583, t33587, t33589, t33591, t33592, t33594)
}
