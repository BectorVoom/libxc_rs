//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1225/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1225<F: Float>(t13929: F, t13932: F, t1383: F, t2425: F, t4506: F, t4508: F, t4769: F, t2478: F, t3975: F, t1309: F, t3974: F, t2497: F, t3966: F, t1328: F, t3965: F, t10030: F, t6725: F) -> (F, F, F, F, F, F, F) {
    let t18177 = 32.0 / 135.0 * t13929;
    let t18178 = 16.0 / 45.0 * t13932;
    let t18180 = 2.0 / 15.0 * t2425 * t1383;
    let t18183 = 32.0 / 45.0 * t4506 * t4508 * t4769;
    let t18184 = t3975 * t2478;
    let t18187 = 16.0 / 45.0 * t3974 * t18184 * t1309;
    let t18188 = t3966 * t2497;
    let t18191 = 16.0 / 45.0 * t3965 * t18188 * t1328;
    let t18192 = t10030 * t6725;
    (t18177, t18178, t18180, t18183, t18187, t18191, t18192)
}
