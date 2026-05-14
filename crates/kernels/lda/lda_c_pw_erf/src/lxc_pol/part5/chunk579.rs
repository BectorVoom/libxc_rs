//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 579/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk579<F: Float>(t164: F, t4137: F, t1590: F, t466: F, t163: F, t2908: F, t148: F, t1198: F, t479: F, t458: F, t1159: F, t695: F, t1: F, t1750: F, t726: F, t1755: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4259 = t4137 * t164;
    let t4260 = 0.00011865309871651405 * t4259;
    let t4261 = t466 * t1590;
    let t4263 = t2908 * t163;
    let t4265 = 0.031505407223141116 * t148 * t4263;
    let t4272 = t1198 * t479;
    let t4275 = 0.09451622166942335 * t458 * t1590;
    let t4276 = t1159 * t164;
    let t4279 = 0.1890324433388467 * t695 * t479;
    let t4291 = t726 * t1750 * t1;
    let t4292 = t4291 * t1755;
    (t4259, t4260, t4261, t4263, t4265, t4272, t4275, t4276, t4279, t4291, t4292)
}
