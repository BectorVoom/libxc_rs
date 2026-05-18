//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 423/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk423<F: Float>(t169: F, t174: F, t171: F, t2629: F, t2630: F, t2635: F, t176: F, t833: F, t44: F, t844: F, t88: F, t194: F, t843: F, t189: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t2639 = piecewise3::<f64>(t170, F::new(0.0), F::new(4.0) / F::new(9.0) * t2629 * t2630 + F::new(4.0) / F::new(3.0) * t171 * t2635);
    let t2640 = t176 * t176;
    let t2641 = F::new(1.0) / t2640;
    let t2642 = t833 * t833;
    let t2645 = -t2635;
    let t2649 = piecewise3::<f64>(t175, F::new(0.0), F::new(4.0) / F::new(9.0) * t2641 * t2642 + F::new(4.0) / F::new(3.0) * t176 * t2645);
    let t2651 = (t2639 + t2649) * t44;
    let t2658 = t88 * t844;
    let t2662 = t843 * t194;
    let t2663 = F::new(1.0) / t2662;
    let t2664 = t189 * t2663;
    (t2640, t2641, t2642, t2645, t2651, t2658, t2663, t2664)
}
