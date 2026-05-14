//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1010/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1010<F: Float>(t5401: F, t595: F, t4010: F, t808: F, t10419: F, t10422: F, t10425: F, t2061: F, t830: F, t11845: F, t2062: F, t11: F, t13290: F, t1349: F, t13368: F, t557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13556 = 4.0 / 5.0 * t5401 * t595;
    let t13558 = 2.0 / 15.0 * t4010 * t808;
    let t13559 = 32.0 / 135.0 * t10419;
    let t13560 = 8.0 / 45.0 * t10422;
    let t13561 = 4.0 / 45.0 * t10425;
    let t13562 = t2061 * t830;
    let t13564 = t11845 * t2062;
    let t13568 = t11 * t1349 * t13290;
    let t13571 = t11 * t557 * t13368;
    (t13556, t13558, t13559, t13560, t13561, t13562, t13564, t13568, t13571)
}
