//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1161/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1161<F: Float>(t439: F, t5482: F, t6364: F, t2010: F, t6371: F, t1444: F, t7535: F, t493: F, t6751: F, t6765: F, t12617: F, t6769: F) -> (F, F, F, F, F) {
    let t20950 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t5482 * t6364;
    let t20953 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2010 * t5482 * t6371;
    let t20955 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1444 * t7535;
    let t20958 = t493 * t6751 * t6765 / F::cast_from(9.0_f64);
    let t20961 = t493 * t12617 * t6769 / F::cast_from(9.0_f64);
    (t20950, t20953, t20955, t20958, t20961)
}
