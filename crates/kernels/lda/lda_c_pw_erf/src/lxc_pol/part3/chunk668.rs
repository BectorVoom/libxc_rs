//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 668/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk668<F: Float>(t185: F, t4039: F, t1401: F, t1403: F, t549: F, t1466: F, t1318: F, t1333: F, t212: F) -> (F, F, F, F, F) {
    let t4041 = F::new(16.0) / F::new(405.0) * t185 * t4039;
    let t4043 = t1401 * t549 * t1403;
    let t4044 = t1466 * t4043;
    let t4046 = F::new(8.0) / F::new(5.0) * t1318 * t4044;
    let t4048 = F::new(1.0) / t212 / t1333;
    (t4041, t4043, t4044, t4046, t4048)
}
