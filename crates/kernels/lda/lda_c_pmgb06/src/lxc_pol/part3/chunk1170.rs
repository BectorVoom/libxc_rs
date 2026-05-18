//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1170/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1170<F: Float>(t1420: F, t5345: F, t486: F, t5102: F, t1499: F, t2018: F, t132: F, t443: F, t459: F, t4828: F, t464: F, t4680: F) -> (F, F, F, F, F) {
    let t13970 = F::new(2.0) / F::new(15.0) * t1420 * t5345;
    let t13971 = t486 * t5102;
    let t13972 = F::new(2.0) / F::new(15.0) * t13971;
    let t13973 = t1499 * t2018;
    let t13974 = t13973 / F::new(15.0);
    let t13978 = F::new(2.0) / F::new(15.0) * t132 * t4828 * t459 * t443;
    let t13979 = t4680 * t464;
    (t13970, t13972, t13974, t13978, t13979)
}
