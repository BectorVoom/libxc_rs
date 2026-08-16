//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1045/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1045(t222: f64, t15462: f64, t1624: f64, t3473: f64, t294: f64, t12924: f64, t295: f64, t559: f64, t3181: f64, t3274: f64, t1100: f64, t3366: f64, t1130: f64, t15176: f64, t15179: f64, t15181: f64, t15183: f64, t15187: f64, t15191: f64, t15195: f64, t15198: f64, t15212: f64, t15214: f64, t15226: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t15463 = 3.0_f64 * t15462;
    let t15464 = t3473 * t1624;
    let t15465 = t294 * t15464;
    let t15466 = 3.0_f64 / 16.0_f64 * t15465;
    let t15467 = piecewise3(t223, 0.0_f64, t12924);
    let t15468 = t295 * t15467;
    let t15469 = t15468 * t559;
    let t15470 = t294 * t15469;
    let t15471 = t15470 / 16.0_f64;
    let t15472 = t3181 * t3274;
    let t15473 = 3.0_f64 * t15472;
    let t15484 = t3366 * t1100;
    let t15488 = -0.2089325e-1_f64 * t15176 - 0.2089325e-1_f64 * t15179 + 0.55715333333333333331e-1_f64 * t15181 + 0.27857666666666666666e-1_f64 * t15183 - 0.41786499999999999999e-1_f64 * t15187 + 0.69644166666666666665e-2_f64 * t15191 + 0.65001222222222222219e-1_f64 * t15195 - 0.65001222222222222219e-1_f64 * t15198 + 0.41786499999999999999e-1_f64 * t15212 - 0.55715333333333333331e-1_f64 * t15214 - 0.579e0_f64 * t15484 * t1130 - 0.72223580246913580243e-1_f64 * t15226;
    (t15463, t15466, t15471, t15473, t15488)
}
