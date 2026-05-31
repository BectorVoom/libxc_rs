//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1121/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1121<F: Float>(t1531: F, t1593: F, t12521: F, t5077: F, t13007: F, t5091: F, t12555: F, t5095: F, t13002: F, t5084: F, t1386: F, t3290: F, t822: F) -> (F, F, F, F, F) {
    let t13308 = t1593 * t1531;
    let t13311 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t13308 * t12521;
    let t13312 = t13007 * t5091;
    let t13313 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13312;
    let t13314 = t12555 * t5095;
    let t13315 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13314;
    let t13318 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5077 * t5084 * t13002;
    let t13322 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t3290 * t822 * t1386;
    (t13311, t13313, t13315, t13318, t13322)
}
