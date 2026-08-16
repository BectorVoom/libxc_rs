//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1080/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1080(t4248: f64, t8461: f64, t7732: f64, t1843: f64, t8460: f64, t651: f64, t7889: f64, t4147: f64, t7933: f64, t1559: f64, t31756: f64, t4364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33577 = t4248 * t8461;
    let t33578 = 2.0_f64 * t33577;
    let t33579 = t7732 * t8461;
    let t33580 = 2.0_f64 * t33579;
    let t33581 = t1843 * t8460;
    let t33582 = t651 * t33581;
    let t33583 = 2.0_f64 * t33582;
    let t33643 = t4248 * t8460;
    let t33644 = 2.0_f64 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0_f64 * t33645;
    let t33651 = t4147 * t7933;
    let t33674 = t4364 * t31756 * t1559;
    (t33578, t33580, t33581, t33583, t33644, t33645, t33646, t33651, t33674)
}
