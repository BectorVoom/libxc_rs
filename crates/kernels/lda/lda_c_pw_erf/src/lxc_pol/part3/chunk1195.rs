//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1195/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1195<F: Float>(t181: F, t184: F, t3464: F, t786: F, t509: F, t944: F, t511: F, t5129: F, t4724: F, t1397: F, t5211: F, t1498: F, t2067: F) -> (F, F, F, F, F, F) {
    let t14066 = F::new(4.0) / F::new(15.0) * t3464 * t181 * t184 * t786;
    let t14070 = F::new(4.0) / F::new(5.0) * t944 * t509 * t184 * t786;
    let t14072 = F::new(2.0) / F::new(5.0) * t511 * t5129;
    let t14074 = F::new(8.0) / F::new(15.0) * t511 * t4724;
    let t14075 = t5211 * t1397;
    let t14076 = F::new(16.0) / F::new(15.0) * t14075;
    let t14078 = F::new(2.0) / F::new(5.0) * t1498 * t2067;
    (t14066, t14070, t14072, t14074, t14076, t14078)
}
