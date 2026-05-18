//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1078/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1078<F: Float>(t169: F, t632: F, t7868: F, t11323: F, t19972: F, t19973: F, t19976: F, t19977: F, t19978: F, t19979: F, t19980: F, t19981: F, t19982: F, t19983: F, t19984: F, t19985: F, t8168: F, t8177: F, t8188: F) -> (F, F) {
    let t20185 = t169 * t7868 * t632;
    let t20188 = t19972 - t19973 - t19976 - t8168 - t8177 - t19977 - t19978 - t19979 + t19980 + t19981 + t11323 - t19982 - t19983 - t19984 + t19985 - t8188;
    (t20185, t20188)
}
