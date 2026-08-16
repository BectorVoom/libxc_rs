//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 768/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk768<F: Float>(t4995: F, t5028: F, t582: F, t186: F, t211: F, t2072: F, t2104: F, t1284: F, t1386: F, t2120: F, t1287: F, t209: F) -> (F, F, F, F, F, F, F, F) {
    let t5029 = t4995 + t5028;
    let t5030 = t582 * t5029;
    let t5031 = t186 * t5030;
    let t5033 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t5031;
    let t5035 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2104 * t2072;
    let t5037 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1284 * t2072;
    let t5039 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2120 * t1386;
    let t5040 = t1287 * t209;
    (t5029, t5030, t5031, t5033, t5035, t5037, t5039, t5040)
}
