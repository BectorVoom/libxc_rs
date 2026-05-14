//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 718/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk718<F: Float>(t4029: F, t4031: F, t4033: F, t2099: F, t514: F, t185: F, t1394: F, t795: F, t2104: F, t2137: F, t1284: F, t1298: F, t2127: F, t2131: F, t2134: F, t511: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5181 = 8.0 / 135.0 * t4029;
    let t5182 = 4.0 / 45.0 * t4031;
    let t5183 = 8.0 / 45.0 * t4033;
    let t5184 = t514 * t2099;
    let t5186 = 8.0 / 45.0 * t185 * t5184;
    let t5188 = 4.0 / 15.0 * t795 * t1394;
    let t5190 = 16.0 / 45.0 * t2104 * t2137;
    let t5192 = 16.0 / 45.0 * t1284 * t2137;
    let t5194 = 16.0 / 45.0 * t1298 * t2127;
    let t5196 = 8.0 / 15.0 * t1298 * t2131;
    let t5198 = 8.0 / 45.0 * t511 * t2134;
    (t5181, t5182, t5183, t5184, t5186, t5188, t5190, t5192, t5194, t5196, t5198)
}
