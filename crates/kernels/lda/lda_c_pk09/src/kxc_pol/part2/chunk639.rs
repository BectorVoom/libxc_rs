//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 639/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk639<F: Float>(t1665: F, t1972: F, t1672: F, t1982: F, t6319: F, t6325: F, t6464: F, t1884: F, t1887: F, t533: F, t6236: F, t529: F, t1803: F, t6477: F, t6501: F, t6505: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6829 = t1972 * t1665;
    let t6831 = t1982 * t1672;
    let t6836 = 0.9421211958699838 * t6319;
    let t6838 = 0.6280807972466558 * t6325;
    let t6844 = 0.20936026574888528 * t6464;
    let t6849 = t1884 * t1887;
    let t6852 = 1.0 / t533 / t6236;
    let t6853 = t529 * t6852;
    let t6864 = t1803 * t6477;
    let t6873 = 0.22687409291590604 * t6501;
    let t6874 = 0.22687409291590604 * t6505;
    (t6829, t6831, t6836, t6838, t6844, t6849, t6853, t6864, t6873, t6874)
}
