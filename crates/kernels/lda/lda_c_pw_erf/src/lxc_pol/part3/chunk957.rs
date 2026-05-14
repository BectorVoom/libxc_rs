//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 957/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk957<F: Float>(t12697: F, t1325: F, t1326: F, t2954: F, t5264: F, t1318: F, t1319: F, t2967: F, t5229: F, t10463: F, t2006: F, t2171: F, t3953: F, t4026: F, t835: F, t1896: F, t603: F) -> (F, F, F, F, F, F, F) {
    let t12698 = 8.0 / 9.0 * t12697;
    let t12702 = 16.0 / 15.0 * t1325 * t1326 * t5264 * t2954;
    let t12706 = 16.0 / 15.0 * t1318 * t1319 * t5229 * t2967;
    let t12708 = t1325 * t10463 * t2006;
    let t12709 = 16.0 / 135.0 * t12708;
    let t12711 = 4.0 / 9.0 * t2171 * t3953;
    let t12713 = 2.0 / 15.0 * t4026 * t835;
    let t12714 = t1896 * t603;
    (t12698, t12702, t12706, t12709, t12711, t12713, t12714)
}
