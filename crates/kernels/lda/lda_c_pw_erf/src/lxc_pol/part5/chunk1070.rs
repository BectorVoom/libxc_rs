//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1070/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1070<F: Float>(t18025: F, t1318: F, t3854: F, t7821: F, t2146: F, t6236: F, t1325: F, t1326: F, t494: F, t7647: F, t1313: F, t519: F, t542: F, t348: F, t7639: F, t7635: F) -> (F, F, F, F, F, F, F, F) {
    let t22263 = 32.0 / 27.0 * t18025;
    let t22265 = t1318 * t3854 * t7821;
    let t22266 = 16.0 / 45.0 * t22265;
    let t22267 = t2146 * t6236;
    let t22268 = 16.0 / 45.0 * t22267;
    let t22272 = 16.0 / 15.0 * t1325 * t1326 * t7647 * t494;
    let t22276 = 8.0 / 15.0 * t519 * t1313 * t7647 * t542;
    let t22277 = t7639 * t348;
    let t22280 = 32.0 / 15.0 * t519 * t1326 * t22277;
    let t22281 = t7635 * t348;
    (t22263, t22266, t22268, t22272, t22276, t22277, t22280, t22281)
}
