//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3191/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191<F: Float>(t1214: F, t17784: F, t1042: F, t1122: F, t1222: F, t1247: F, t1250: F, t12621: F, t12794: F, t12809: F, t12953: F, t13102: F, t16771: F, t17505: F, t17547: F, t17736: F, t247: F, t3591: F, t3626: F, t3718: F, t3719: F, t3720: F, t44675: F, t44678: F, t44681: F, t471: F, t482: F, t5308: F, t5312: F, t5332: F, t5351: F, t5373: F, t5384: F, t5391: F, t56157: F, t56161: F, t56165: F, t56192: F, t56196: F, t56555: F, t57780: F, t57786: F, t58730: F) -> (F, F) {
    let t58760 = t17784 * t1214;
    let t58772 = -F::cast_from(0.85748036236139473944e-3_f64) * t57780 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t16771 * t1122 + F::cast_from(0.30488190661738479624e-2_f64) * t57786 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t247 * t3719 * t56555 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1042 * t482 * t58730 * t1250 - F::cast_from(0.22866142996303859718e-2_f64) * t17505 * t12953 - F::cast_from(0.28582678745379824648e-3_f64) * t44675 - F::cast_from(0.34299214494455789577e-2_f64) * t17547 * t3591 - F::cast_from(0.57165357490759649295e-3_f64) * t44678 + F::cast_from(0.57165357490759649295e-3_f64) * t44681 + t5373 * t12794 / F::cast_from(18.0_f64) - t1222 * t5308 * t56157 / F::cast_from(16.0_f64) - t1222 * t5308 * t56161 / F::cast_from(144.0_f64) - t1222 * t5308 * t56165 / F::cast_from(12.0_f64) + t1222 * t5312 * t56192 / F::cast_from(72.0_f64) + t1222 * t5312 * t56196 / F::cast_from(72.0_f64) + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t5332 * t58760 + F::cast_from(0.33875767401931644027e-2_f64) * t5391 * t13102 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t5351 * t471 * t12621;
    (t58760, t58772)
}
