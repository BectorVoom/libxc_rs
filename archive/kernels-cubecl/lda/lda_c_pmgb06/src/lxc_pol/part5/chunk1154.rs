//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1154/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1154<F: Float>(t1444: F, t7640: F, t1450: F, t493: F, t7639: F, t19395: F, t496: F, t498: F, t2470: F, t5305: F, t1972: F, t6282: F) -> (F, F, F, F, F) {
    let t20863 = t1444 * t7640 / F::cast_from(45.0_f64);
    let t20866 = t493 * t1450 * t7639 / F::cast_from(45.0_f64);
    let t20870 = t493 * t496 * t498 * t19395 / F::cast_from(45.0_f64);
    let t20872 = t5305 * t2470 / F::cast_from(9.0_f64);
    let t20874 = t1972 * t6282 / F::cast_from(9.0_f64);
    (t20863, t20866, t20870, t20872, t20874)
}
