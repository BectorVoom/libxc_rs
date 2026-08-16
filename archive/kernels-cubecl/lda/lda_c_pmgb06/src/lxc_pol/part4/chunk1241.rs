//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1241/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1241<F: Float>(t16343: F, t446: F, t1427: F, t6127: F, t1989: F, t5305: F, t2493: F, t3213: F, t1963: F, t5187: F, t1083: F, t6502: F) -> (F, F, F, F, F, F) {
    let t16345 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16343 * t446;
    let t16347 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6127 * t1427;
    let t16349 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5305 * t1989;
    let t16350 = t3213 * t2493;
    let t16351 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t16350;
    let t16353 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5187 * t1963;
    let t16354 = t6502 * t1083;
    (t16345, t16347, t16349, t16351, t16353, t16354)
}
