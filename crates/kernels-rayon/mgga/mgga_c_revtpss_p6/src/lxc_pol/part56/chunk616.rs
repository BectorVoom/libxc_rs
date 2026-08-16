//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 616/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk616(t1892: f64, t212: f64, t1358: f64, t689: f64, t1893: f64, t786: f64, t1364: f64, t1889: f64, t3989: f64, t1882: f64, t550: f64, t543: f64) -> (f64, f64, f64, f64) {
    let t5599 = t212 * t1892;
    let t5600 = t5599 * t1358;
    let t5601 = t689 * t5600;
    let t5603 = t786 * t1893;
    let t5604 = t5603 * t1364;
    let t5606 = t3989 * t1889;
    let t5608 = t550 * t1882;
    let t5609 = t5608 * t543;
    (t5601, t5604, t5606, t5609)
}
