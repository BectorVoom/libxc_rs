//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1067/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1067<F: Float>(t12306: F, t12304: F, t12311: F, t12313: F, t18225: F, t19971: F, t19972: F, t19975: F, t19976: F, t19977: F, t19978: F, t19979: F, t12448: F, t12450: F, t12461: F, t12463: F, t19983: F, t19985: F, t19986: F, t19987: F, t19988: F, t19992: F, t19995: F) -> (F, F) {
    let t21942 = 1.2e-20 * t12306;
    let t21944 = -t19971 - t19972 - t19975 - t19976 + 2.0 * t12304 + t21942 + t12311 + t12313 - t19977 + t19978 - 2.0 / 9.0 * t18225 - t19979;
    let t21948 = -t12448 - t12450 + t19983 - t19985 - t12461 - t12463 + t19986 + t19987 + t19988 + t19992 + t19995;
    (t21944, t21948)
}
