//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 516/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk516<F: Float>(t174: F, t176: F, t2641: F, t2642: F, t2645: F, t2639: F, t44: F, t230: F, t838: F, t908: F, t844: F, t88: F, t194: F, t843: F, t189: F, t850: F, t851: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t2649 = piecewise3(t175, 0.0, 4.0 / 9.0 * t2641 * t2642 + 4.0 / 3.0 * t176 * t2645);
    let t2651 = (t2639 + t2649) * t44;
    let t2652 = t2651 * t230;
    let t2653 = t838 * t908;
    let t2658 = t88 * t844;
    let t2662 = t843 * t194;
    let t2663 = 1.0 / t2662;
    let t2664 = t189 * t2663;
    let t2665 = t850 * t850;
    let t2666 = t2665 * t851;
    (t2651, t2652, t2653, t2658, t2663, t2664, t2665, t2666)
}
