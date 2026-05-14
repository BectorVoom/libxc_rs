//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1024/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1024<F: Float>(t17109: F, t17112: F, t17114: F, t2134: F, t2407: F, t17117: F, t12475: F, t6442: F, t6762: F, t2325: F, t806: F, t494: F, t3965: F, t4494: F, t12314: F, t6756: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21388 = 8.0 / 15.0 * t17109;
    let t21389 = 8.0 / 15.0 * t17112;
    let t21390 = 16.0 / 15.0 * t17114;
    let t21391 = t2407 * t2134;
    let t21392 = 8.0 / 15.0 * t21391;
    let t21393 = 4.0 / 45.0 * t17117;
    let t21396 = 64.0 / 15.0 * t12475 * t6762 * t6442;
    let t21397 = t2325 * t806;
    let t21398 = t21397 * t494;
    let t21401 = 16.0 / 15.0 * t3965 * t4494 * t21398;
    let t21403 = 16.0 / 15.0 * t12314 * t6756;
    (t21388, t21389, t21390, t21392, t21393, t21396, t21397, t21398, t21401, t21403)
}
