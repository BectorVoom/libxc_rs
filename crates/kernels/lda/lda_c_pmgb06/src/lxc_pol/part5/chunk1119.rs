//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1119/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1119<F: Float>(t1966: F, t2064: F, t439: F, t6554: F, t20420: F, t20423: F, t20428: F, t20431: F, t20435: F, t20436: F, t20438: F, t20440: F, t20442: F, t20445: F) -> (F, F) {
    let t20449 = t439 * t1966 * t6554 * t2064 / F::cast_from(5.0_f64);
    let t20450 = -t20420 - t20423 + t20428 - t20431 + t20435 - t20436 - t20438 - t20440 + t20442 + t20445 + t20449;
    (t20449, t20450)
}
