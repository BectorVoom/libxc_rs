//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1103/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1103(t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t3172: f64, t4874: f64, t3127: f64) -> (f64, f64, f64, f64) {
    let t15745 = t4857 * t1053;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15752 = t11922 * t4901;
    let t15754 = 0.28582678745379824648e-3_f64 * t4899 * t15752;
    let t15769 = t3172 * t4874;
    let t15771 = 0.19055119163586549765e-3_f64 * t3127 * t15769;
    (t15745, t15750, t15754, t15771)
}
