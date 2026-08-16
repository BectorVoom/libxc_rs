//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1029/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1029(t136: f64, t243: f64, t2371: f64, t94: f64, t197: f64, t531: f64, t2013: f64, t1450: f64, t3889: f64, t2242: f64, t607: f64, t640: f64, t644: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14685 = t243 * t136;
    let t18163 = t94 * t2371;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    let t25089 = t1450 * t3889;
    let t25102 = t2242 * t607;
    let t25110 = t77 * t640 * t644;
    (t14685, t18163, t25081, t25082, t25089, t25102, t25110)
}
