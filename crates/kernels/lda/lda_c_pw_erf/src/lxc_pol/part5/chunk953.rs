//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 953/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk953<F: Float>(t197: F, t3892: F, t3518: F, t2120: F, t3550: F, t3553: F, t795: F, t4505: F, t668: F, t3667: F, t573: F, t3437: F, t822: F) -> (F, F, F, F, F, F) {
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12046 = t2120 * t3550;
    let t12047 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12046;
    let t12050 = t795 * t3553;
    let t12051 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12050;
    let t12064 = t4505 * t668;
    let t12071 = t573 * t3667;
    let t12083 = t822 * t3437;
    (t12031, t12047, t12051, t12064, t12071, t12083)
}
