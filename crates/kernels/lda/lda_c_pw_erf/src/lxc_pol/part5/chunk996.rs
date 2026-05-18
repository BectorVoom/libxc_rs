//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 996/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk996<F: Float>(t1746: F, t7314: F, t1034: F, t2343: F, t40: F, t344: F, t6071: F, t1064: F, t2344: F, t1067: F, t6069: F, t479: F, t7032: F) -> (F, F, F, F, F, F, F) {
    let t15421 = t7314 * t1746;
    let t15450 = t40 * t2343 * t1034;
    let t15453 = t344 * t6071;
    let t15455 = t1064 * t2344;
    let t15457 = t1067 * t2344;
    let t15461 = t344 * t6069;
    let t15481 = t7032 * t479;
    (t15421, t15450, t15453, t15455, t15457, t15461, t15481)
}
