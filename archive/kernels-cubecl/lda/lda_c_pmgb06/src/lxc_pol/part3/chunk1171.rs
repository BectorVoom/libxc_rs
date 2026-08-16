//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1171/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1171<F: Float>(t132: F, t137: F, t13979: F, t477: F, t1423: F, t5350: F, t12389: F, t1897: F, t439: F, t1385: F, t3010: F, t5271: F) -> (F, F, F, F) {
    let t13983 = t132 * t137 * t13979 * t477 / F::cast_from(10.0_f64);
    let t13984 = t1423 * t5350;
    let t13985 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13984;
    let t13988 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t439 * t1897 * t12389;
    let t13992 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t1385 * t5271 * t3010;
    (t13983, t13985, t13988, t13992)
}
