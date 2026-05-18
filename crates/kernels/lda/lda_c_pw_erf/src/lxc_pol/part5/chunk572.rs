//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 572/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk572<F: Float>(t133: F, t3227: F, t153: F, t274: F, t2869: F, t1125: F, t678: F, t1459: F, t529: F, t1283: F, t518: F) -> (F, F, F, F, F) {
    let t3349 = t133 * t3227;
    let t3373 = F::new(4.429070076315393) * t153 * t2869 * t274;
    let t3378 = t153 * t1125 * t678;
    let t3402 = t1459 * t529;
    let t3416 = t1283 * t518;
    (t3349, t3373, t3378, t3402, t3416)
}
