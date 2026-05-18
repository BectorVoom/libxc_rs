//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 801/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk801<F: Float>(t168: F, t7831: F, t96: F, t2262: F, t694: F, t891: F, t609: F, t3767: F, t623: F, t896: F, t633: F, t903: F) -> (F, F, F, F, F, F) {
    let t8052 = t7831 * t168;
    let t8053 = t96 * t8052;
    let t8061 = t891 * t2262 * t694;
    let t8065 = t891 * t2262 * t609;
    let t8066 = t3767 * t8065;
    let t8069 = t896 * t2262 * t623;
    let t8073 = t903 * t2262 * t633;
    (t8053, t8061, t8065, t8066, t8069, t8073)
}
