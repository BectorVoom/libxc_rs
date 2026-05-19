//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 809/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk809<F: Float>(t12: F, t176: F, t7616: F, t166: F, t161: F, t2389: F, t764: F, t7300: F, t44: F, t131: F, t178: F, t7549: F, t7553: F, t7557: F, t7561: F, t7565: F, t7569: F, t7571: F, t7573: F, t7576: F, t7579: F, t7583: F, t7587: F, t7589: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t7617 = t7616 * t176;
    let t7618 = t166 * t7617;
    let t7620 = t161 * t7618 / F::new(30.0);
    let t7621 = t764 * t2389;
    let t7626 = piecewise3::<F>(t13, F::new(0.0), F::new(2.0) * t12 * t7300 + F::new(6.0) * t7621);
    let t7627 = t7626 * t44;
    let t7628 = t7627 * t131;
    let t7630 = t7628 * t178 / F::new(30.0);
    let t7631 = t7549 - t7553 - t7557 + t7561 + t7565 + t7569 + t7571 + t7573 + t7576 + t7579 + t7583 + t7587 + t7589 + t7620 + t7630;
    (t7617, t7618, t7620, t7621, t7627, t7628, t7630, t7631)
}
