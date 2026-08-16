//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1054/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1054(t10727: f64, t2723: f64, t10726: f64, t2661: f64, t231: f64, t2662: f64, t221: f64, t2430: f64, t2675: f64, t2674: f64, t2735: f64, t2783: f64) -> (f64, f64, f64, f64) {
    let t10728 = t10727 * t2723;
    let t10729 = t10726 * t10728;
    let t10730 = t2661 * t10729;
    let t10732 = t10727 * t231;
    let t10733 = t2662 * t10732;
    let t10734 = t2661 * t10733;
    let t10741 = t2675 * t221 * t2430;
    let t10742 = t2674 * t10741;
    let t10744 = t2735 * t2783;
    (t10730, t10734, t10742, t10744)
}
