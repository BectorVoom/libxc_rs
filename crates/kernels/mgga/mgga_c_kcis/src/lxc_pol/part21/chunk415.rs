//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 415/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk415<F: Float>(t169: F, t174: F, t171: F, t2629: F, t2630: F, t2635: F, t176: F, t833: F, t44: F, t844: F, t88: F, t194: F, t843: F, t189: F, t850: F, t851: F, t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t2639 = piecewise3(t170, 0.0, 4.0 / 9.0 * t2629 * t2630 + 4.0 / 3.0 * t171 * t2635);
    let t2640 = t176 * t176;
    let t2641 = 1.0 / t2640;
    let t2642 = t833 * t833;
    let t2645 = -t2635;
    let t2649 = piecewise3(t175, 0.0, 4.0 / 9.0 * t2641 * t2642 + 4.0 / 3.0 * t176 * t2645);
    let t2651 = (t2639 + t2649) * t44;
    let t2658 = t88 * t844;
    let t2662 = t843 * t194;
    let t2663 = 1.0 / t2662;
    let t2664 = t189 * t2663;
    let t2665 = t850 * t850;
    let t2666 = t2665 * t851;
    let t2675 = -0.78438333333333333333e0 * t2318 + 0.15687666666666666667e1 * t2321 + 0.68863333333333333333e0 * t2323 + 0.14025833333333333333e0 * t2327 + 0.28051666666666666667e0 * t2329 + 0.17365833333333333333e0 * t2331;
    (t2640, t2641, t2642, t2645, t2651, t2658, t2663, t2664, t2665, t2666, t2675)
}
