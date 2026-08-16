//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1074/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1074(t1497: f64, t8441: f64, t7714: f64, t8621: f64, t1493: f64, t84: f64, t4248: f64, t8460: f64, t7889: f64, t4147: f64, t7933: f64, t1559: f64, t31756: f64, t4364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33612 = t8441 * t1497;
    let t33620 = t8441 * t7714;
    let t33621 = t8621 * t33620;
    let t33624 = t84 * t1493;
    let t33643 = t4248 * t8460;
    let t33644 = 2.0_f64 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0_f64 * t33645;
    let t33651 = t4147 * t7933;
    let t33674 = t4364 * t31756 * t1559;
    (t33612, t33620, t33621, t33624, t33644, t33646, t33651, t33674)
}
