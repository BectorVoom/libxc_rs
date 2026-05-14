//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 549/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk549<F: Float>(t1529: F, t565: F, t2070: F, t220: F, t211: F, t191: F, t717: F, t187: F, t190: F, t1244: F, t43: F) -> (F, F, F, F, F, F) {
    let t3433 = t565 * t1529;
    let t3437 = t2070 * t220;
    let t3439 = 16.0 / 405.0 * t211 * t3437;
    let t3469 = t717 * t191;
    let t3472 = 0.02962962962962963 * t190 * t3469 * t187;
    let t3476 = 1.0 / t1244 / t43;
    (t3433, t3437, t3439, t3469, t3472, t3476)
}
