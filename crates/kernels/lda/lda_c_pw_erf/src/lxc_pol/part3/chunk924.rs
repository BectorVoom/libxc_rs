//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 924/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk924<F: Float>(t1446: F, t4862: F, t11701: F, t1326: F, t519: F, t11705: F, t4829: F, t2031: F, t3709: F, t5244: F, t4850: F, t1313: F, t3545: F, t789: F, t10467: F, t2030: F) -> (F, F, F, F, F, F, F, F) {
    let t12178 = 32.0 / 15.0 * t1446 * t4862;
    let t12181 = 8.0 / 45.0 * t519 * t1326 * t11701;
    let t12184 = 16.0 / 15.0 * t519 * t4829 * t11705;
    let t12186 = 4.0 / 15.0 * t3709 * t2031;
    let t12188 = 4.0 / 15.0 * t1446 * t5244;
    let t12190 = 16.0 / 15.0 * t1446 * t4850;
    let t12194 = 4.0 / 45.0 * t519 * t1313 * t789 * t3545;
    let t12196 = t519 * t10467 * t2030;
    (t12178, t12181, t12184, t12186, t12188, t12190, t12194, t12196)
}
