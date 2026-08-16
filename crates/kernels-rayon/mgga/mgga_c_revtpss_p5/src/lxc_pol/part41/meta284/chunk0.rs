//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1040/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1040(t2665: f64, t9775: f64, t2681: f64, t820: f64, t849: f64, t857: f64, t240: f64, t2719: f64, t2735: f64, t2783: f64, t2664: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    let t10744 = t2735 * t2783;
    let t10745 = t808 * t2664;
    (t10719, t10722, t10723, t10726, t10744, t10745)
}
