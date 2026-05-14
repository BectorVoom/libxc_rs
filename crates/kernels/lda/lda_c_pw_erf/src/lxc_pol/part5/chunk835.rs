//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 835/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk835<F: Float>(t1191: F, t465: F, t1138: F, t1597: F, t1578: F, t2910: F, t485: F, t4259: F, t142: F, t450: F, t1128: F, t1159: F, t285: F, t2872: F, t695: F, t1063: F, t274: F) -> (F, F, F, F, F, F, F, F) {
    let t10810 = t1191 * t465;
    let t10812 = t10810 * t1138 * t1597;
    let t10816 = 0.03950778065781896 * t1578 * t2910 * t485;
    let t10817 = 0.7561297733553868 * t4259;
    let t10832 = t450 * t142;
    let t10868 = t1159 * t1128 * t285;
    let t10872 = 0.0011622696607154768 * t695 * t2872 * t285;
    let t10878 = 6.399008129061525 * t1063 * t274;
    (t10810, t10812, t10816, t10817, t10832, t10868, t10872, t10878)
}
