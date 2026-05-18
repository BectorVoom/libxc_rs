//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 607/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk607<F: Float>(t198: F, t3464: F, t186: F, t493: F, t191: F, t717: F, t187: F, t190: F, t1272: F, t331: F, t1244: F, t43: F) -> (F, F, F, F, F, F, F) {
    let t3465 = t198 * t3464;
    let t3466 = t186 * t3465;
    let t3468 = F::new(4.0) / F::new(15.0) * t493 * t3466;
    let t3469 = t717 * t191;
    let t3472 = F::new(0.02962962962962963) * t190 * t3469 * t187;
    let t3473 = t331 * t1272;
    let t3476 = F::new(1.0) / t1244 / t43;
    (t3465, t3466, t3468, t3469, t3472, t3473, t3476)
}
