//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 719/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk719(t1364: f64, t5603: f64, t1889: f64, t3989: f64, t1882: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t1413: f64, t1868: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5604 = t5603 * t1364;
    let t5606 = t3989 * t1889;
    let t5608 = t550 * t1882;
    let t5609 = t5608 * t543;
    let t5610 = t3992 * t5609;
    let t5611 = t2661 * t5610;
    let t5617 = t1413 * t1868;
    let t5618 = t547 * t5617;
    (t5604, t5606, t5609, t5610, t5611, t5617, t5618)
}
