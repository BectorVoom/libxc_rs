//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 866/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk866(t1358: f64, t13725: f64, t2439: f64, t5622: f64, t9765: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t1882: f64, t4003: f64, t1873: f64, t9741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13765 = t9765 * t5622;
    let t13779 = t9775 * t5610;
    let t13781 = t9779 * t1889;
    let t13790 = t1882 * t4003;
    let t13798 = t9741 * t1873;
    (t13727, t13765, t13779, t13781, t13790, t13798)
}
