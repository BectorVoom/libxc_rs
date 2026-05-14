//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 955/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk955<F: Float>(t11883: F, t11897: F, t467: F, t452: F, t1971: F, t2825: F, t132: F, t2824: F, t93: F, t2070: F, t2758: F, t11129: F, t477: F, t2812: F, t7248: F, t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t6873: F, t6874: F, t6878: F) -> (F, F, F, F, F, F, F) {
    let t11898 = t11883 + t11897;
    let t11899 = t467 * t11898;
    let t11900 = t11899 * t452;
    let t11903 = t2825 * t1971;
    let t11906 = t132 * t2824;
    let t11907 = t93 * t11906;
    let t11910 = t2070 * t2758;
    let t11913 = t11129 * t477;
    let t11915 = t2812 * t7248;
    let t11936 = 0.04525483399593904 * t11066 + 0.09050966799187808 * t10959 + 0.4537481858318121 * t11529 - 0.4537481858318121 * t11532 - 0.4537481858318121 * t11535 + 0.6806222787477182 * t11539 - 0.4537481858318121 * t11542 + 0.04525483399593904 * t11076 + t6873 + 0.015084944665313014 * t11073 + t6878 - 0.015084944665313014 * t6337 - 0.04525483399593904 * t6323 + 0.15124939527727072 * t6550 + t6874 - 0.15124939527727072 * t6508 + 0.015084944665313014 * t6467;
    (t11900, t11903, t11907, t11910, t11913, t11915, t11936)
}
