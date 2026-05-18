//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1224/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1224<F: Float>(t1318: F, t2065: F, t5269: F, t6242: F, t558: F, t7513: F, t352: F, t571: F, t9286: F, t2146: F, t6375: F, t6385: F) -> (F, F, F, F) {
    let t22102 = F::new(16.0) / F::new(5.0) * t1318 * t5269 * t6242 * t2065;
    let t22103 = t7513 * t558;
    let t22107 = F::new(8.0) / F::new(15.0) * t571 * t9286 * t22103 * t352;
    let t22109 = F::new(4.0) / F::new(9.0) * t2146 * t6375;
    let t22111 = F::new(32.0) / F::new(27.0) * t2146 * t6385;
    (t22102, t22107, t22109, t22111)
}
