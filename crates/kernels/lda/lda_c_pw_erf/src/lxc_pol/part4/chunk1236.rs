//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1236/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1236<F: Float>(t4753: F, t6483: F, t2385: F, t9752: F, t3416: F, t6479: F, t1976: F, t2098: F, t4848: F, t519: F, t1325: F, t2497: F, t494: F, t5289: F, t542: F, t1318: F, t2526: F, t5269: F, t549: F, t593: F) -> (F, F, F, F, F, F, F) {
    let t18363 = 64.0 / 45.0 * t4753 * t6483;
    let t18365 = 16.0 / 45.0 * t9752 * t2385;
    let t18367 = 32.0 / 45.0 * t3416 * t6479;
    let t18369 = 64.0 / 45.0 * t3416 * t6483;
    let t18373 = 32.0 / 45.0 * t519 * t4848 * t1976 * t2098;
    let t18378 = 16.0 / 15.0 * t1325 * t5289 * t2497 * t494 * t542;
    let t18383 = 16.0 / 15.0 * t1318 * t5269 * t2526 * t549 * t593;
    (t18363, t18365, t18367, t18369, t18373, t18378, t18383)
}
