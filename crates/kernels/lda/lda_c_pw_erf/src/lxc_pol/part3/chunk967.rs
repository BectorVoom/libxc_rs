//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 967/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk967<F: Float>(t1210: F, t645: F, t646: F, t3940: F, t656: F, t3943: F, t1410: F, t1416: F, t1419: F, t1423: F, t3936: F, t10967: F, t168: F, t270: F) -> (F, F, F, F, F, F, F) {
    let t11159 = F::cast_from(0.05402469135802469_f64) * t645 * t1210 * t646;
    let t11160 = t3940 * t656;
    let t11162 = t3943 * t656;
    let t11164 = t1416 * t1410;
    let t11166 = t1419 * t1410;
    let t11168 = t1423 * t3936;
    let t11196 = F::cast_from(0.9079060239445599_f64) * t168 * t10967 * t270;
    (t11159, t11160, t11162, t11164, t11166, t11168, t11196)
}
