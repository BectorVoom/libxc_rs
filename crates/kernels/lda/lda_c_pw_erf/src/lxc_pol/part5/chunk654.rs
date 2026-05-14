//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 654/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk654<F: Float>(t571: F, t6236: F, t2384: F, t3854: F, t1318: F, t811: F, t833: F, t593: F, t5269: F, t2035: F, t4763: F, t2011: F, t2146: F, t2014: F, t2018: F, t2419: F, t549: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6237 = t571 * t6236;
    let t6238 = 16.0 / 135.0 * t6237;
    let t6239 = t3854 * t2384;
    let t6240 = t1318 * t6239;
    let t6241 = 32.0 / 135.0 * t6240;
    let t6242 = t811 * t833;
    let t6243 = t6242 * t593;
    let t6244 = t5269 * t6243;
    let t6246 = 16.0 / 15.0 * t1318 * t6244;
    let t6248 = 16.0 / 45.0 * t4763 * t2035;
    let t6250 = 8.0 / 45.0 * t2146 * t2011;
    let t6252 = 16.0 / 45.0 * t2146 * t2014;
    let t6254 = 8.0 / 27.0 * t2146 * t2018;
    let t6255 = t2419 * t549;
    (t6237, t6238, t6239, t6240, t6241, t6242, t6243, t6244, t6246, t6248, t6250, t6252, t6254, t6255)
}
