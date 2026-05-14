//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk768<F: Float>(t5: F, t12: F, t1941: F, t2381: F, t330: F, t594: F, t7284: F, t7290: F, t1949: F, t2389: F, t336: F, t598: F, t7295: F, t7300: F, t44: F, t224: F, t3007: F, t4070: F, t7687: F, t7689: F, t7692: F, t7694: F, t7698: F, t7700: F, t7701: F, t7702: F, t7703: F, t7707: F, t7708: F, t7713: F, t7717: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t7959 = piecewise3(t6, 0.0, 80.0 / 27.0 * t330 * t7284 + 40.0 / 3.0 * t1941 * t2381 + 8.0 / 3.0 * t594 * t7290);
    let t7967 = piecewise3(t13, 0.0, 80.0 / 27.0 * t336 * t7295 + 40.0 / 3.0 * t1949 * t2389 + 8.0 / 3.0 * t598 * t7300);
    let t7970 = (t7959 / 2.0 + t7967 / 2.0) * t44;
    let t7973 = t7687 + t7689 + t7692 + t7694 + t7698 - t7970 * t224 / 15.0 + t7700 + t7701 - t7702 - t7703 - t7707 - t7708 + t3007 + t7713 + t7717 + t4070;
    (t7970, t7973)
}
