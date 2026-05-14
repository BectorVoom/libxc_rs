//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 968/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk968<F: Float>(t4259: F, t142: F, t450: F, t1089: F, t1191: F, t169: F, t301: F, t1726: F, t1729: F, t285: F, t4120: F, t477: F, t1128: F, t1159: F, t2872: F, t695: F) -> (F, F, F, F, F, F, F) {
    let t10817 = 0.7561297733553868 * t4259;
    let t10832 = t450 * t142;
    let t10838 = t169 * t1191 * t1089 * t301;
    let t10840 = t1729 * t1726;
    let t10865 = t4120 * t477 * t285;
    let t10868 = t1159 * t1128 * t285;
    let t10872 = 0.0011622696607154768 * t695 * t2872 * t285;
    (t10817, t10832, t10838, t10840, t10865, t10868, t10872)
}
