//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1182/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1182<F: Float>(t13924: F, t1298: F, t4568: F, t2162: F, t571: F, t9432: F, t3899: F, t5374: F, t1466: F, t2161: F, t3655: F, t10030: F, t5167: F) -> (F, F, F, F, F, F) {
    let t13925 = F::new(8.0) / F::new(45.0) * t13924;
    let t13926 = t1298 * t4568;
    let t13927 = F::new(4.0) / F::new(3.0) * t13926;
    let t13929 = t571 * t9432 * t2162;
    let t13930 = F::new(8.0) / F::new(45.0) * t13929;
    let t13932 = t571 * t3899 * t5374;
    let t13933 = F::new(8.0) / F::new(15.0) * t13932;
    let t13937 = F::new(4.0) / F::new(15.0) * t571 * t1466 * t2161 * t3655;
    let t13938 = t10030 * t5167;
    (t13925, t13927, t13930, t13933, t13937, t13938)
}
