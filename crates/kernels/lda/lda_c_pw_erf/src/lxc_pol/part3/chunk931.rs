//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 931/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk931<F: Float>(t4688: F, t954: F, t4758: F, t571: F, t2178: F, t3709: F, t4804: F, t5394: F, t1450: F, t5327: F, t518: F, t5214: F, t1443: F, t1287: F, t1318: F, t1466: F, t5315: F) -> (F, F, F, F, F, F, F, F) {
    let t12289 = t4688 * t954;
    let t12292 = 16.0 / 15.0 * t571 * t4758 * t12289;
    let t12294 = 8.0 / 15.0 * t3709 * t2178;
    let t12296 = 4.0 / 5.0 * t4804 * t5394;
    let t12297 = t5327 * t1450;
    let t12298 = 16.0 / 45.0 * t12297;
    let t12299 = t5214 * t518;
    let t12301 = 8.0 / 5.0 * t12299 * t1443;
    let t12305 = 4.0 / 5.0 * t1318 * t1466 * t5315 * t1287;
    (t12289, t12292, t12294, t12296, t12298, t12299, t12301, t12305)
}
