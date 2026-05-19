//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 757/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk757<F: Float>(t15216: F, t15450: F, t218: F, t1009: F, t3179: F, t1053: F, t3181: F, t3274: F, t1100: F, t3366: F, t1130: F, t15176: F, t15179: F, t15181: F, t15183: F, t15187: F, t15191: F, t15195: F, t15198: F, t15212: F, t15214: F, t15226: F) -> (F, F, F, F) {
    let t15451 = t15216 + t15450;
    let t15452 = t15451 * t218;
    let t15461 = t3179 * t1009;
    let t15462 = t15461 * t1053;
    let t15463 = F::new(3.0) * t15462;
    let t15472 = t3181 * t3274;
    let t15473 = F::new(3.0) * t15472;
    let t15484 = t3366 * t1100;
    let t15488 = -F::new(0.2089325e-1) * t15176 - F::new(0.2089325e-1) * t15179 + F::cast_from(0.55715333333333333331e-1_f64) * t15181 + F::cast_from(0.27857666666666666666e-1_f64) * t15183 - F::cast_from(0.41786499999999999999e-1_f64) * t15187 + F::cast_from(0.69644166666666666665e-2_f64) * t15191 + F::cast_from(0.65001222222222222219e-1_f64) * t15195 - F::cast_from(0.65001222222222222219e-1_f64) * t15198 + F::cast_from(0.41786499999999999999e-1_f64) * t15212 - F::cast_from(0.55715333333333333331e-1_f64) * t15214 - F::new(0.579e0) * t15484 * t1130 - F::cast_from(0.72223580246913580243e-1_f64) * t15226;
    (t15452, t15463, t15473, t15488)
}
