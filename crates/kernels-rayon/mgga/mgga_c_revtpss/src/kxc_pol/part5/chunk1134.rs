//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1134/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1134(t5284: f64, t73: f64, t17350: f64, t3767: f64, t372: f64, t5277: f64, t1285: f64, t12865: f64, t15904: f64, t3623: f64, t13148: f64, t3172: f64, t5303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17633 = t5284 * t73;
    let t17654 = t3767 * t17350;
    let t17661 = t372 * t5277;
    let t17693 = t1285 * t12865;
    let t17708 = t3623 * t15904;
    let t17709 = t13148 * t17708;
    let t17720 = t3172 * t5303;
    (t17633, t17654, t17661, t17693, t17708, t17709, t17720)
}
