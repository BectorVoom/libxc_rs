//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 671/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk671<F: Float>(t1210: F, t168: F, t861: F, t153: F, t1891: F, t474: F, t1729: F, t452: F, t454: F, t1872: F, t2765: F, t1184: F, t780: F) -> (F, F, F, F, F) {
    let t5907 = t168 * t1210 * t861;
    let t5911 = F::cast_from(1.1389037339096726_f64) * t153 * t474 * t1891;
    let t5924 = t1729 * t452 * t454;
    let t5925 = t2765 * t1872;
    let t5931 = t1184 * t780;
    (t5907, t5911, t5924, t5925, t5931)
}
