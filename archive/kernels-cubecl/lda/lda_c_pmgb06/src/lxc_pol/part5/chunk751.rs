//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 751/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk751<F: Float>(t69: F, t6983: F, t6986: F, t2695: F, t342: F, t2209: F, t769: F, t2448: F, t2247: F, t2248: F, t3505: F, t3517: F, t3525: F, t3644: F, t5874: F, t6980: F, t7017: F, t7024: F, t7026: F) -> (F, F, F, F, F, F) {
    let t7069 = t69 * t6983;
    let t7071 = t69 * t6986;
    let t7073 = t2695 * t342;
    let t7077 = t769 * t2209;
    let t7081 = t2448 * t342;
    let t7085 = -t7017 - t3505 - F::cast_from(0.7663355555555555_f64) * t3644 - t3517 + t3525 - F::cast_from(1.724255_f64) * t69 * t6980 - F::cast_from(1.724255_f64) * t7069 + F::cast_from(0.5747516666666667_f64) * t7071 - F::cast_from(20.69106_f64) * t2247 * t5874 * t7073 + F::cast_from(10.34553_f64) * t2247 * t2248 * t7077 + F::cast_from(5.172765_f64) * t2247 * t2248 * t7081 + t7024 - t7026;
    (t7069, t7071, t7073, t7077, t7081, t7085)
}
