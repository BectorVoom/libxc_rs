//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 889/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk889(t240: f64, t2719: f64, t243: f64, t2722: f64, t2723: f64, t2661: f64, t231: f64, t2662: f64, t10489: f64, t828: f64, t855: f64, t221: f64, t2430: f64, t2675: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10726 = t2719 * t240;
    let t10727 = t243 * t2722;
    let t10728 = t10727 * t2723;
    let t10729 = t10726 * t10728;
    let t10730 = t2661 * t10729;
    let t10732 = t10727 * t231;
    let t10733 = t2662 * t10732;
    let t10734 = t2661 * t10733;
    let t10737 = t855 * t828 * t10489;
    let t10741 = t2675 * t221 * t2430;
    (t10728, t10730, t10732, t10734, t10737, t10741)
}
