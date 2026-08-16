//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3112/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112(t15061: f64, t50819: f64, t11361: f64, t11365: f64, t1137: f64, t11420: f64, t1148: f64, t1155: f64, t1156: f64, t15126: f64, t15136: f64, t15146: f64, t15179: f64, t15219: f64, t15229: f64, t18603: f64, t3332: f64, t3333: f64, t3334: f64, t3357: f64, t3359: f64, t3377: f64, t3401: f64, t44188: f64, t4840: f64, t4862: f64, t51371: f64, t51385: f64, t51651: f64, t51677: f64, t6037: f64, t6053: f64, t6069: f64, t6085: f64, t6088: f64, t64261: f64, t64292: f64, t64425: f64, t64433: f64, t64436: f64) -> (f64, f64) {
    let t64441 = 0.38596750796862084161e3_f64 * t50819 * t15061;
    let t64442 = -4.0_f64 * t3332 * t64261 * t1137 - 0.14035736694323150897e2_f64 * t11365 * t6069 * t3377 - 24.0_f64 * t11420 * t6037 * t3333 + 12.0_f64 * t15146 * t15229 + 0.35089341735807877242e1_f64 * t3401 * t6085 * t3377 + 6.0_f64 * t3357 * t6053 * t3333 - 0.46785788981077169656e1_f64 * t51677 * t4840 + 0.69263436422725855034e2_f64 * t51371 * t4862 - 0.46785788981077169656e1_f64 * t15136 * t15179 + 0.69263436422725855034e2_f64 * t15126 * t15219 + 0.70178683471615754484e1_f64 * t11361 * t18603 + 0.64327917994770140268e2_f64 * t3357 * t64261 * t3359 - 2.0_f64 * t64292 * t3334 + 0.5848223622634646207e0_f64 * t1148 * t64425 * t1156 + 0.17315859105681463759e2_f64 * t44188 * t6088 - t64433 - t64436 + 0.41016075432865626631e4_f64 * t51385 * t51651 * t1155 + t64441;
    (t64441, t64442)
}
