//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1222/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1222<F: Float>(t607: F, t7970: F, t19718: F, t19722: F, t19724: F, t19726: F, t19727: F, t19729: F, t19731: F, t19733: F, t19736: F, t19738: F, t19739: F) -> F {
    let t21928 = t7970 * t607;
    let t21930 = -t19718 - t19722 - F::new(2.0) / F::new(45.0) * t21928 - t19724 - t19726 - t19727 + t19729 - t19731 - t19733 - t19736 - t19738 - t19739;
    t21930
}
