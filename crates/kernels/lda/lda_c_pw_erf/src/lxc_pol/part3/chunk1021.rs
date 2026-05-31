//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1021/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1021<F: Float>(t9276: F, t9280: F, t9306: F, t9315: F, t9318: F, t9338: F, t9340: F, t3416: F, t5272: F, t1318: F, t2065: F, t5269: F, t549: F, t593: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11960 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t9276;
    let t11961 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9280;
    let t11962 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9306;
    let t11963 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t9315;
    let t11964 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t9318;
    let t11965 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9338;
    let t11966 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t9340;
    let t11968 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t3416 * t5272;
    let t11973 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1318 * t5269 * t2065 * t549 * t593;
    (t11960, t11961, t11962, t11963, t11964, t11965, t11966, t11968, t11973)
}
