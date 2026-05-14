//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 938/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk938<F: Float>(t12323: F, t3965: F, t494: F, t6710: F, t1251: F, t4489: F, t12324: F, t4488: F, t10015: F, t5138: F, t10030: F, t5152: F, t12064: F, t4509: F, t108: F, t267: F, t564: F) -> (F, F, F, F, F, F) {
    let t12402 = 16.0 / 15.0 * t3965 * t6710 * t12323 * t494;
    let t12403 = t4489 * t1251;
    let t12406 = 16.0 / 15.0 * t4488 * t12403 * t12324;
    let t12408 = 16.0 / 15.0 * t10015 * t5138;
    let t12409 = t10030 * t5152;
    let t12410 = 32.0 / 45.0 * t12409;
    let t12411 = t12064 * t4509;
    let t12412 = 32.0 / 45.0 * t12411;
    let t12414 = t564 * t108 * t267;
    (t12402, t12406, t12408, t12410, t12412, t12414)
}
