//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1141/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1141<F: Float>(t1308: F, t2065: F, t2415: F, t571: F, t504: F, t7792: F, t1313: F, t348: F, t519: F, t16127: F, t16129: F, t16134: F) -> (F, F, F, F, F) {
    let t21051 = F::new(8.0) / F::new(15.0) * t571 * t1308 * t2415 * t2065;
    let t21052 = t7792 * t504;
    let t21056 = F::new(4.0) / F::new(45.0) * t519 * t1313 * t21052 * t348;
    let t21057 = F::new(64.0) / F::new(45.0) * t16127;
    let t21058 = F::new(32.0) / F::new(45.0) * t16129;
    let t21059 = F::new(16.0) / F::new(15.0) * t16134;
    (t21051, t21056, t21057, t21058, t21059)
}
