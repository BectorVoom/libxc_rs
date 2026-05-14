//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1057/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1057<F: Float>(t12299: F, t1313: F, t13523: F, t1472: F, t15694: F, t16209: F, t2146: F, t2171: F, t2393: F, t2397: F, t34: F, t4848: F, t519: F, t5327: F, t6280: F, t6380: F, t6409: F, t6414: F, t6447: F, t6461: F, t6465: F, t739: F, t7816: F) -> (F,) {
    let t22013 = -16.0 / 135.0 * t13523 - 8.0 / 15.0 * t5327 * t2397 - 8.0 / 15.0 * t2171 * t6461 - 16.0 / 15.0 * t2171 * t6465 - 4.0 / 15.0 * t1472 * t7816 - 8.0 / 15.0 * t519 * t4848 * t6280 * t34 + 16.0 / 15.0 * t15694 * t2393 + 16.0 / 15.0 * t12299 * t2393 - 4.0 / 15.0 * t519 * t1313 * t16209 * t739 + 8.0 / 15.0 * t2146 * t6409 + 8.0 / 5.0 * t2146 * t6414 - 8.0 / 3.0 * t2146 * t6380 + 32.0 / 15.0 * t2146 * t6447;
    (t22013,)
}
