//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1289/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1289<F: Float>(t1217: F, t2455: F, t11020: F, t11022: F, t11025: F, t11027: F, t11029: F, t16054: F, t16056: F, t16059: F, t16063: F, t16066: F, t16067: F, t16070: F, t16073: F, t16076: F, t16078: F, t16082: F) -> (F,) {
    let t19123 = t2455 * t1217;
    let t19127 = -t16054 - t16056 + t16059 + t16063 - t16066 + 2.0 / 135.0 * t19123 + t16067 + 0.19947266666666666 * t11020 - 0.13298177777777778 * t11022 - t11025 + t11027 + t11029 - t16070 + t16073 + t16076 - t16078 - t16082;
    (t19127,)
}
