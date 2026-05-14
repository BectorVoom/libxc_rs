//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 881/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk881<F: Float>(t2497: F, t529: F, t494: F, t1440: F, t1325: F, t1390: F, t542: F, t519: F, t2401: F, t518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6997 = t529 * t2497;
    let t6998 = t6997 * t494;
    let t6999 = t1440 * t6998;
    let t7001 = 4.0 / 15.0 * t1325 * t6999;
    let t7002 = t1390 * t2497;
    let t7003 = t7002 * t542;
    let t7004 = t1440 * t7003;
    let t7006 = 4.0 / 15.0 * t519 * t7004;
    let t7007 = t2401 * t518;
    (t6997, t6998, t6999, t7001, t7002, t7003, t7004, t7006, t7007)
}
