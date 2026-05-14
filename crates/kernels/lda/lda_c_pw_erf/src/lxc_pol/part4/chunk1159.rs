//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1159/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1159<F: Float>(t184: F, t786: F, t793: F, t944: F, t12717: F, t1980: F, t494: F, t1234: F, t2407: F, t9645: F, t12723: F, t16003: F, t542: F, t12380: F, t4488: F, t12031: F, t12362: F, t15825: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17073 = 8.0 / 15.0 * t944 * t793 * t184 * t786;
    let t17074 = 16.0 / 45.0 * t12717;
    let t17078 = 16.0 / 15.0 * t494 * t1980 * t184 * t786;
    let t17079 = t2407 * t1234;
    let t17080 = 16.0 / 45.0 * t17079;
    let t17081 = 16.0 / 405.0 * t9645;
    let t17082 = 32.0 / 45.0 * t12723;
    let t17083 = t16003 * t542;
    let t17086 = 64.0 / 81.0 * t4488 * t12380 * t17083;
    let t17089 = 256.0 / 81.0 * t12362 * t12031 * t15825;
    (t17073, t17074, t17078, t17080, t17081, t17082, t17083, t17086, t17089)
}
