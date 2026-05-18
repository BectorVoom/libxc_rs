//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 405/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk405<F: Float>(t1508: F, t199: F, t494: F, t181: F, t184: F) -> (F, F, F, F) {
    let t1510 = F::new(2.0) / F::new(15.0) * t1508 * t199;
    let t1511 = t494 * t494;
    let t1512 = t1511 * t181;
    let t1513 = t1512 * t184;
    (t1510, t1511, t1512, t1513)
}
