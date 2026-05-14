//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 910/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk910<F: Float>(t9315: F, t9318: F, t9338: F, t9340: F, t3416: F, t5272: F, t1318: F, t2065: F, t5269: F, t549: F, t593: F, t1287: F, t833: F, t1381: F, t5270: F, t1466: F, t3667: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11963 = 16.0 / 135.0 * t9315;
    let t11964 = 8.0 / 45.0 * t9318;
    let t11965 = 16.0 / 45.0 * t9338;
    let t11966 = 32.0 / 45.0 * t9340;
    let t11968 = 16.0 / 5.0 * t3416 * t5272;
    let t11973 = 16.0 / 5.0 * t1318 * t5269 * t2065 * t549 * t593;
    let t11978 = 8.0 / 5.0 * t1318 * t5269 * t833 * t1287 * t593;
    let t11982 = 8.0 / 5.0 * t1318 * t5269 * t5270 * t1381;
    let t11983 = t1466 * t3667;
    (t11963, t11964, t11965, t11966, t11968, t11973, t11978, t11982, t11983)
}
