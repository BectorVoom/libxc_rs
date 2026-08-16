//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3112/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112<F: Float>(t15061: F, t50819: F, t11361: F, t11365: F, t1137: F, t11420: F, t1148: F, t1155: F, t1156: F, t15126: F, t15136: F, t15146: F, t15179: F, t15219: F, t15229: F, t18603: F, t3332: F, t3333: F, t3334: F, t3357: F, t3359: F, t3377: F, t3401: F, t44188: F, t4840: F, t4862: F, t51371: F, t51385: F, t51651: F, t51677: F, t6037: F, t6053: F, t6069: F, t6085: F, t6088: F, t64261: F, t64292: F, t64425: F, t64433: F, t64436: F) -> (F, F) {
    let t64441 = F::cast_from(0.38596750796862084161e3_f64) * t50819 * t15061;
    let t64442 = -F::cast_from(4.0_f64) * t3332 * t64261 * t1137 - F::cast_from(0.14035736694323150897e2_f64) * t11365 * t6069 * t3377 - F::cast_from(24.0_f64) * t11420 * t6037 * t3333 + F::cast_from(12.0_f64) * t15146 * t15229 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t6085 * t3377 + F::cast_from(6.0_f64) * t3357 * t6053 * t3333 - F::cast_from(0.46785788981077169656e1_f64) * t51677 * t4840 + F::cast_from(0.69263436422725855034e2_f64) * t51371 * t4862 - F::cast_from(0.46785788981077169656e1_f64) * t15136 * t15179 + F::cast_from(0.69263436422725855034e2_f64) * t15126 * t15219 + F::cast_from(0.70178683471615754484e1_f64) * t11361 * t18603 + F::cast_from(0.64327917994770140268e2_f64) * t3357 * t64261 * t3359 - F::cast_from(2.0_f64) * t64292 * t3334 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t64425 * t1156 + F::cast_from(0.17315859105681463759e2_f64) * t44188 * t6088 - t64433 - t64436 + F::cast_from(0.41016075432865626631e4_f64) * t51385 * t51651 * t1155 + t64441;
    (t64441, t64442)
}
