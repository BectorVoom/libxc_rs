//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1485/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1485(t3211: f64, t4845: f64, t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t1028: f64, t11779: f64, t11792: f64, t11994: f64, t15724: f64, t15725: f64, t15728: f64, t15732: f64, t15736: f64, t1665: f64, t4839: f64, t4875: f64) -> f64 {
    let t15744 = 0.15244095330869239812e-2_f64 * t3211 * t4845;
    let t15745 = t4857 * t1053;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15752 = t11922 * t4901;
    let t15754 = 0.28582678745379824648e-3_f64 * t4899 * t15752;
    let t15755 = t15724 + 0.85748036236139473944e-3_f64 * t15725 * t4839 - 0.45732285992607719436e-2_f64 * t15728 * t4839 - 0.47637797908966374413e-4_f64 * t15732 - t15736 - 0.28582678745379824648e-3_f64 * t11994 * t4875 + 0.22866142996303859718e-2_f64 * t11792 * t1665 - 0.72409452821628889107e-2_f64 * t11779 * t1665 + t15744 + 0.22866142996303859718e-2_f64 * t15745 * t1028 + 0.47637797908966374413e-4_f64 * t15750 - t15754;
    t15755
}
