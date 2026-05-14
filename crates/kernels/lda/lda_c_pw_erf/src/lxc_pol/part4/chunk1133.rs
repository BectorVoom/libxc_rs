//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1133/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1133<F: Float>(t2497: F, t4489: F, t1314: F, t4488: F, t1351: F, t2526: F, t3832: F, t571: F, t951: F, t4763: F, t5226: F, t5231: F, t1313: F, t519: F, t6280: F, t945: F) -> (F, F, F, F, F) {
    let t16657 = t4489 * t2497;
    let t16660 = 16.0 / 45.0 * t4488 * t16657 * t1314;
    let t16665 = 4.0 / 27.0 * t571 * t3832 * t2526 * t1351 * t951;
    let t16667 = 16.0 / 45.0 * t4763 * t5226;
    let t16669 = 16.0 / 27.0 * t4763 * t5231;
    let t16673 = 4.0 / 45.0 * t519 * t1313 * t6280 * t945;
    (t16660, t16665, t16667, t16669, t16673)
}
