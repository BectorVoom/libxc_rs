//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3191/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191(t1214: f64, t17784: f64, t1042: f64, t1122: f64, t1222: f64, t1247: f64, t1250: f64, t12621: f64, t12794: f64, t12809: f64, t12953: f64, t13102: f64, t16771: f64, t17505: f64, t17547: f64, t17736: f64, t247: f64, t3591: f64, t3626: f64, t3718: f64, t3719: f64, t3720: f64, t44675: f64, t44678: f64, t44681: f64, t471: f64, t482: f64, t5308: f64, t5312: f64, t5332: f64, t5351: f64, t5373: f64, t5384: f64, t5391: f64, t56157: f64, t56161: f64, t56165: f64, t56192: f64, t56196: f64, t56555: f64, t57780: f64, t57786: f64, t58730: f64) -> (f64, f64) {
    let t58760 = t17784 * t1214;
    let t58772 = -0.85748036236139473944e-3_f64 * t57780 - 0.17149607247227894789e-2_f64 * t17736 * t3626 * t16771 * t1122 + 0.30488190661738479624e-2_f64 * t57786 + 0.12862205435420921092e-2_f64 * t5384 * t247 * t3719 * t56555 + 0.21437009059034868486e-3_f64 * t1247 * t1042 * t482 * t58730 * t1250 - 0.22866142996303859718e-2_f64 * t17505 * t12953 - 0.28582678745379824648e-3_f64 * t44675 - 0.34299214494455789577e-2_f64 * t17547 * t3591 - 0.57165357490759649295e-3_f64 * t44678 + 0.57165357490759649295e-3_f64 * t44681 + t5373 * t12794 / 18.0_f64 - t1222 * t5308 * t56157 / 16.0_f64 - t1222 * t5308 * t56161 / 144.0_f64 - t1222 * t5308 * t56165 / 12.0_f64 + t1222 * t5312 * t56192 / 72.0_f64 + t1222 * t5312 * t56196 / 72.0_f64 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t5332 * t58760 + 0.33875767401931644027e-2_f64 * t5391 * t13102 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t5351 * t471 * t12621;
    (t58760, t58772)
}
