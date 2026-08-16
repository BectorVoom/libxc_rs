//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2755/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2755(t10175: f64, t22399: f64, t13734: f64, t1904: f64, t689: f64, t2453: f64, t3908: f64, t6889: f64, t22398: f64, t2470: f64, t3915: f64, t22452: f64, t9680: f64) -> (f64, f64, f64, f64, f64) {
    let t73647 = t10175 * t22399;
    let t73652 = t689 * t13734 * t1904;
    let t73656 = t2453 * t6889 * t3908;
    let t73662 = t3915 * t22398 * t2470;
    let t73666 = t9680 * t22452 * t2470;
    (t73647, t73652, t73656, t73662, t73666)
}
