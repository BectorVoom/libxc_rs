//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 971/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk971<F: Float>(t1423: F, t6413: F, t6416: F, t6419: F, t1447: F, t6399: F, t6403: F, t6504: F, t5499: F, t6407: F, t161: F, t489: F, t6448: F) -> (F, F, F, F, F, F, F, F) {
    let t16029 = t1423 * t6413;
    let t16031 = t1423 * t6416;
    let t16033 = t1423 * t6419;
    let t16051 = t1447 * t6399;
    let t16053 = t1447 * t6403;
    let t16055 = t1447 * t6504;
    let t16057 = t5499 * t6407;
    let t16089 = t161 * t489 * t6448;
    (t16029, t16031, t16033, t16051, t16053, t16055, t16057, t16089)
}
