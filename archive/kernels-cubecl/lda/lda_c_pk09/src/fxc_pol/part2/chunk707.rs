//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 707/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk707<F: Float>(t6464: F, t1884: F, t1887: F, t533: F, t6236: F, t529: F, t1803: F, t6477: F, t6501: F, t6505: F, t6522: F, t6319: F) -> (F, F, F, F, F, F, F, F) {
    let t6844 = F::cast_from(0.20936026574888528_f64) * t6464;
    let t6849 = t1884 * t1887;
    let t6852 = F::cast_from(1.0_f64) / t533 / t6236;
    let t6853 = t529 * t6852;
    let t6864 = t1803 * t6477;
    let t6873 = F::cast_from(0.22687409291590604_f64) * t6501;
    let t6874 = F::cast_from(0.22687409291590604_f64) * t6505;
    let t6878 = F::cast_from(0.30249879055454143_f64) * t6522;
    let t6882 = F::cast_from(0.04525483399593904_f64) * t6319;
    (t6844, t6849, t6853, t6864, t6873, t6874, t6878, t6882)
}
