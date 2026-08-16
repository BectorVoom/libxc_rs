//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1259/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1259(t15648: f64, t373: f64, t371: f64, t372: f64, t4742: f64, t993: f64, t225: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t4852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15649 = t373 * t15648;
    let t15651 = t371 * t372 * t15649;
    let t15654 = t4742 * t993;
    let t15655 = t15654 * t225;
    let t15656 = t15655 * t366;
    let t15662 = 0.28582678745379824648e-3_f64 * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    (t15651, t15654, t15655, t15656, t15662, t15666)
}
