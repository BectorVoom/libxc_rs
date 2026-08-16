//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 836/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk836(t13926: f64, t543: f64, t13790: f64, t1398: f64, t1558: f64, t836: f64, t231: f64, t2723: f64, t136: f64, t243: f64, t220: f64, t125: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14224 = t13926 * t543;
    let t14230 = t13790 * t1398;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14586 = t1558 * t2723;
    let t14587 = t14586 * t836;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14691 = t125 * t4343;
    (t14224, t14230, t14495, t14587, t14685, t14686, t14691)
}
