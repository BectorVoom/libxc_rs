//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 660/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk660<F: Float>(t23: F, t3588: F, t1321: F, t67: F, t386: F, t374: F) -> (F, F, F, F) {
    let t4038 = t3588 * t23;
    let t4042 = F::cast_from(1.0_f64) / t1321 / t67;
    let t4043 = t386 * t4042;
    let t4044 = t374 * t374;
    (t4038, t4042, t4043, t4044)
}
