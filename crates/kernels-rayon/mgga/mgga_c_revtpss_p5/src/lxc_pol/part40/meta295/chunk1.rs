//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1053/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1053(t10716: f64, t2677: f64, t2665: f64, t9775: f64, t2681: f64, t820: f64, t849: f64, t857: f64, t240: f64, t2719: f64, t243: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10717 = t10716 * t2677;
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    let t10727 = t243 * t2722;
    (t10717, t10719, t10722, t10723, t10726, t10727)
}
