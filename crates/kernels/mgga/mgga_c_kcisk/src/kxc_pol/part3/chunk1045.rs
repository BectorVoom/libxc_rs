//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1045/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1045<F: Float>(t222: F, t15462: F, t1624: F, t3473: F, t294: F, t12924: F, t295: F, t559: F, t3181: F, t3274: F, t1100: F, t3366: F, t1130: F, t15176: F, t15179: F, t15181: F, t15183: F, t15187: F, t15191: F, t15195: F, t15198: F, t15212: F, t15214: F, t15226: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t15463 = F::cast_from(3.0_f64) * t15462;
    let t15464 = t3473 * t1624;
    let t15465 = t294 * t15464;
    let t15466 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t15465;
    let t15467 = piecewise3::<F>(t223, F::cast_from(0.0_f64), t12924);
    let t15468 = t295 * t15467;
    let t15469 = t15468 * t559;
    let t15470 = t294 * t15469;
    let t15471 = t15470 / F::cast_from(16.0_f64);
    let t15472 = t3181 * t3274;
    let t15473 = F::cast_from(3.0_f64) * t15472;
    let t15484 = t3366 * t1100;
    let t15488 = -F::cast_from(0.2089325e-1_f64) * t15176 - F::cast_from(0.2089325e-1_f64) * t15179 + F::cast_from(0.55715333333333333331e-1_f64) * t15181 + F::cast_from(0.27857666666666666666e-1_f64) * t15183 - F::cast_from(0.41786499999999999999e-1_f64) * t15187 + F::cast_from(0.69644166666666666665e-2_f64) * t15191 + F::cast_from(0.65001222222222222219e-1_f64) * t15195 - F::cast_from(0.65001222222222222219e-1_f64) * t15198 + F::cast_from(0.41786499999999999999e-1_f64) * t15212 - F::cast_from(0.55715333333333333331e-1_f64) * t15214 - F::cast_from(0.579e0_f64) * t15484 * t1130 - F::cast_from(0.72223580246913580243e-1_f64) * t15226;
    (t15463, t15466, t15471, t15473, t15488)
}
