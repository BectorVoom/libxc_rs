//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1096/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1096<F: Float>(t1464: F, t1639: F, t5071: F, t5138: F, t2865: F, t3032: F, t5077: F, t822: F, t2965: F, t5078: F, t1601: F, t12693: F) -> (F, F, F, F) {
    let t13053 = t1639 * t1464;
    let t13056 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5138 * t13053 * t5071;
    let t13060 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5077 * t3032 * t822 * t2865;
    let t13063 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t5078 * t2965;
    let t13064 = t1601 * t1464;
    let t13067 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5138 * t13064 * t12693;
    (t13056, t13060, t13063, t13067)
}
