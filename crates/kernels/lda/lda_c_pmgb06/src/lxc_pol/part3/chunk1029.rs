//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1029/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1029<F: Float>(t1680: F, t2026: F, t432: F, t4830: F, t132: F, t2851: F, t814: F, t2852: F, t802: F, t3134: F, t824: F, t1554: F, t161: F, t2100: F) -> (F, F, F, F, F, F) {
    let t12227 = t2026 * t1680;
    let t12230 = F::new(2.0) / F::new(15.0) * t432 * t4830;
    let t12232 = t132 * t2851 * t814;
    let t12233 = F::new(4.0) / F::new(405.0) * t12232;
    let t12234 = t802 * t2852;
    let t12235 = F::new(4.0) / F::new(405.0) * t12234;
    let t12237 = t3134 * t824 / F::new(30.0);
    let t12239 = t161 * t1554 * t2100;
    (t12227, t12230, t12233, t12235, t12237, t12239)
}
