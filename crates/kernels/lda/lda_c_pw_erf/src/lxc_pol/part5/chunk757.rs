//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 757/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk757<F: Float>(t523: F, t5992: F, t522: F, t519: F, t1446: F, t2554: F, t1390: F, t2471: F, t494: F, t1440: F, t1325: F, t2532: F, t4753: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6938 = t523 * t5992;
    let t6939 = t522 * t6938;
    let t6941 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t519 * t6939;
    let t6943 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1446 * t2554;
    let t6944 = t1390 * t2471;
    let t6945 = t6944 * t494;
    let t6946 = t1440 * t6945;
    let t6948 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t6946;
    let t6950 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4753 * t2532;
    (t6938, t6939, t6941, t6943, t6944, t6945, t6946, t6948, t6950)
}
