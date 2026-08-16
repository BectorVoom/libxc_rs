//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2167/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2167(t15717: f64, t247: f64, t3116: f64, t11672: f64, t11675: f64, t11712: f64, t11774: f64, t15684: f64, t15689: f64, t15693: f64, t15697: f64, t15700: f64, t15703: f64, t15707: f64, t15712: f64, t15716: f64, t3101: f64, t3106: f64, t3130: f64, t4788: f64, t4831: f64, t4834: f64) -> (f64, f64) {
    let t15719 = t247 * t3116 * t15717;
    let t15722 = -0.15244095330869239812e-2_f64 * t3106 * t4831 - 0.15244095330869239812e-2_f64 * t11672 * t4788 + t15684 + 0.28582678745379824648e-3_f64 * t11675 * t4788 - 0.28582678745379824648e-3_f64 * t15689 * t15693 - 0.28582678745379824648e-3_f64 * t11774 * t15697 - 0.57165357490759649296e-3_f64 * t15700 * t15703 + 0.19055119163586549765e-3_f64 * t11712 - 0.28582678745379824648e-3_f64 * t15707 * t3130 - 0.31758531939310916276e-4_f64 * t15712 - 0.28582678745379824648e-3_f64 * t4834 * t3101 - 0.12862205435420921092e-2_f64 * t15716 * t15719;
    (t15719, t15722)
}
