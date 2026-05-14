//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 575/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk575<F: Float>(t159: F, t285: F, t4137: F, t169: F, t274: F, t2817: F, t301: F, t1131: F, t1586: F, t485: F, t1138: F, t1597: F, t2881: F, t2910: F, t482: F, t1098: F, t2916: F) -> (F, F, F, F, F, F) {
    let t4140 = 0.006715335817467199 * t4137 * t159 * t285;
    let t4144 = 0.9247854820715865 * t169 * t2817 * t274 * t301;
    let t4153 = t1586 * t1131 * t485;
    let t4156 = t2881 * t1138 * t1597;
    let t4160 = 0.005926167098672845 * t482 * t2910 * t485;
    let t4163 = 0.0014862827083471494 * t1098 * t2916 * t1597;
    (t4140, t4144, t4153, t4156, t4160, t4163)
}
