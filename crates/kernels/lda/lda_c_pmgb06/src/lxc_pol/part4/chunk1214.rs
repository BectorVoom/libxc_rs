//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1214/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1214<F: Float>(t1420: F, t6361: F, t1972: F, t5337: F, t1080: F, t6507: F, t1919: F, t493: F, t4602: F, t6407: F, t1981: F, t5447: F, t6406: F) -> (F, F, F, F, F, F) {
    let t16000 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1420 * t6361;
    let t16002 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1972 * t5337;
    let t16003 = t6507 * t1080;
    let t16006 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t493 * t1919 * t16003;
    let t16008 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4602 * t6407;
    let t16011 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1981 * t5447 * t6406;
    (t16000, t16002, t16003, t16006, t16008, t16011)
}
