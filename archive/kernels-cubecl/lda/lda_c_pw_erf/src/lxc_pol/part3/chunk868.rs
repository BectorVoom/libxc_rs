//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 868/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk868<F: Float>(t1022: F, t2986: F, t1012: F, t2983: F, t400: F, t1: F, t2979: F, t397: F, t1023: F, t1054: F, t2946: F, t3111: F) -> (F, F, F, F, F) {
    let t8370 = t2986 * t1022;
    let t8373 = F::cast_from(6152.338212604677_f64) * t400 * t2983 * t1012 * t8370;
    let t8375 = t2979 * t1 * t397;
    let t8382 = F::cast_from(21.053604230838733_f64) * t400 * t1054 * t1023;
    let t8386 = F::cast_from(623.3672123775311_f64) * t400 * t2946 * t1012 * t3111;
    (t8370, t8373, t8375, t8382, t8386)
}
