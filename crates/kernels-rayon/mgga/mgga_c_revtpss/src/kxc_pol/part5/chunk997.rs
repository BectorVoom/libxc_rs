//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 997/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk997(t136: f64, t2476: f64, t2482: f64, t596: f64, t849: f64, t2677: f64, t2665: f64, t9775: f64, t2681: f64, t820: f64, t857: f64, t240: f64, t2719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10703 = t2476 * t136;
    let t10716 = t2482 * t849 * t596;
    let t10717 = t10716 * t2677;
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    (t10703, t10716, t10717, t10719, t10722, t10723, t10726)
}
