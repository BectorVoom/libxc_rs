//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 691/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk691<F: Float>(t1992: F, t6286: F, t493: F, t1444: F, t2489: F, t1450: F, t2488: F, t1420: F, t2493: F, t1426: F, t2492: F, t439: F) -> (F, F, F, F, F, F, F, F) {
    let t6287 = t1992 * t6286;
    let t6289 = t493 * t6287 / F::new(5.0);
    let t6291 = F::new(2.0) / F::new(45.0) * t1444 * t2489;
    let t6292 = t1450 * t2488;
    let t6294 = F::new(2.0) / F::new(45.0) * t493 * t6292;
    let t6296 = F::new(2.0) / F::new(45.0) * t1420 * t2493;
    let t6297 = t1426 * t2492;
    let t6299 = F::new(2.0) / F::new(45.0) * t439 * t6297;
    (t6287, t6289, t6291, t6292, t6294, t6296, t6297, t6299)
}
