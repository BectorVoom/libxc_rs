//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 802/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk802<F: Float>(t439: F, t7525: F, t2002: F, t2501: F, t1972: F, t2497: F, t2500: F, t5482: F, t2496: F, t5486: F, t493: F, t6781: F, t764: F) -> (F, F, F, F, F, F, F, F) {
    let t7527 = t439 * t7525 / F::cast_from(15.0_f64);
    let t7529 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t2501;
    let t7531 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t2497;
    let t7532 = t5482 * t2500;
    let t7534 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t7532;
    let t7535 = t5486 * t2496;
    let t7537 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t7535;
    let t7538 = t6781 * t764;
    (t7527, t7529, t7531, t7532, t7534, t7535, t7537, t7538)
}
