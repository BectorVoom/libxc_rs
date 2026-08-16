//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 314/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk314<F: Float>(t1055: F, t400: F, t1036: F, t1038: F, t1041: F, t1043: F, t1045: F, t1047: F, t1049: F, t1053: F, t916: F, t962: F) -> (F, F) {
    let t1056 = t400 * t1055;
    let t1057 = F::cast_from(17.315755899375862_f64) * t1056;
    let t1058 = t962 + t1036 + t1038 + t1041 - t1043 - t1045 + t1047 + t1049 - t916 - t1053 - t1057;
    (t1057, t1058)
}
