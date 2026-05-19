//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 873/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk873<F: Float>(t286: F, t3951: F, t637: F, t8131: F, t3734: F, t974: F, t1022: F, t1039: F, t232: F, t3669: F, t8595: F, t696: F, t8522: F, t963: F, t967: F) -> (F, F, F, F, F) {
    let t8781 = t637 * t3951 * t286;
    let t8785 = t8131 * t286;
    let t8787 = t974 * t3734;
    let t8794 = F::cast_from(6207.121550312808_f64) * t232 / t1039 / t1022 * t8595 * t3669;
    let t8798 = F::cast_from(51.94757731704439_f64) * t696 * t963 * t8522 * t967;
    (t8781, t8785, t8787, t8794, t8798)
}
