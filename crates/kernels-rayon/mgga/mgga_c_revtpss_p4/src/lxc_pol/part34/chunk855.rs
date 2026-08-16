//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 855/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk855(t14563: f64, t2798: f64, t1568: f64, t2783: f64, t786: f64, t2435: f64, t4519: f64, t1558: f64, t2723: f64, t1531: f64, t37: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    let t14568 = t786 * t14567;
    let t14581 = t2435 * t4519;
    let t14586 = t1558 * t2723;
    let t14613 = t37 * t1531;
    let t14671 = t124 * t1558;
    (t14564, t14568, t14581, t14586, t14613, t14671)
}
