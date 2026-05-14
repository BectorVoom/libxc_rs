//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 985/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk985<F: Float>(t1318: F, t1466: F, t5029: F, t549: F, t581: F, t1476: F, t5334: F, t12874: F, t1469: F, t1325: F, t1390: F, t1392: F, t34: F, t4956: F, t1124: F, t188: F) -> (F, F, F, F, F) {
    let t13162 = 4.0 / 5.0 * t1318 * t1466 * t581 * t5029 * t549;
    let t13163 = t5334 * t1476;
    let t13164 = 16.0 / 45.0 * t13163;
    let t13166 = 8.0 / 5.0 * t12874 * t1469;
    let t13171 = 8.0 / 5.0 * t1325 * t4956 * t1390 * t34 * t1392;
    let t13172 = t1124 * t188;
    (t13162, t13164, t13166, t13171, t13172)
}
