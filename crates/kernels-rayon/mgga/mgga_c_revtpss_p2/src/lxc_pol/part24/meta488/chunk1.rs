//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1482/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482(t2439: f64, t3895: f64, t6919: f64, t2453: f64, t3908: f64, t6889: f64, t22398: f64, t2470: f64, t3915: f64, t22452: f64, t9680: f64, t22409: f64, t2435: f64) -> (f64, f64, f64, f64, f64) {
    let t73641 = t2439 * t3895 * t6919;
    let t73656 = t2453 * t6889 * t3908;
    let t73662 = t3915 * t22398 * t2470;
    let t73666 = t9680 * t22452 * t2470;
    let t73673 = t2435 * t22409;
    (t73641, t73656, t73662, t73666, t73673)
}
