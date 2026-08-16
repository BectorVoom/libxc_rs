//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1354/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1354<F: Float>(t10482: F, t106209: F, t106218: F, t106267: F, t106307: F, t106328: F, t106352: F, t21487: F, t21503: F, t21512: F, t21516: F, t21542: F, t21551: F, t21570: F, t21580: F, t21594: F, t21609: F, t23419: F, t23537: F, t23541: F, t25580: F, t25683: F, t28582: F, t28587: F, t3131: F, t360: F, t5875: F, t5900: F, t5909: F, t6717: F, t6742: F, t6744: F, t6765: F, t68: F, t7583: F, t82987: F, t82989: F, t83028: F, t83196: F, t88321: F, t88336: F, t88342: F, t88479: F, t88513: F, t88594: F, t99483: F, t99495: F, t99497: F, t99501: F, t99507: F, t99647: F, t99680: F, t99687: F, t99720: F) -> F {
    let t106355 = -t88479 / F::cast_from(2304.0_f64) - t88336 / F::cast_from(432.0_f64) - t88321 / F::cast_from(3456.0_f64) + t6765 * t21609 / F::cast_from(384.0_f64) + t23537 * t21487 / F::cast_from(256.0_f64) - t23541 * t21503 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t6765 * t21512 - t6765 * t21551 / F::cast_from(384.0_f64) + t88594 * t5875 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t6765 * t21516 + F::cast_from(0.30279567070605293142e-3_f64) * t25683 * t28587 - F::cast_from(0.30279567070605293142e-3_f64) * t88342 * t28582 + F::cast_from(0.30279567070605293142e-3_f64) * t99720 * t7583 + t88513 * t5909 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t23419 * t21570 - t25580 * t5900 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t6765 * t21580 + t6717 * t21542 / F::cast_from(288.0_f64) + t83028 - t99501 / F::cast_from(576.0_f64) + t99680 / F::cast_from(576.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t6744 * t21594 * t68 * t360 + t99483 / F::cast_from(384.0_f64) + t99687 / F::cast_from(768.0_f64) + t99495 / F::cast_from(384.0_f64) + t106352 - F::cast_from(0.30279567070605293142e-3_f64) * t99647 + t99497 / F::cast_from(576.0_f64) + t106328 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t99507 + t106218 + t106307 + F::cast_from(0.60559134141210586284e-3_f64) * t82987 * t82989 * t106209 * t10482 - F::cast_from(0.60559134141210586284e-3_f64) * t82987 * t83196 * t106209 * t3131 + t106267;
    t106355
}
