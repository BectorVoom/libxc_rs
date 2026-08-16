//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1115/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1115<F: Float>(t20391: F, t5068: F, t5069: F, t12546: F, t19314: F, t13304: F, t5138: F, t5139: F, t12529: F, t13300: F, t5077: F, t5078: F, t6364: F) -> (F, F, F, F, F, F) {
    let t20394 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t5069 * t20391;
    let t20397 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5068 * t12546 * t19314;
    let t20400 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5138 * t13304 * t19314;
    let t20403 = t5138 * t5139 * t20391 / F::cast_from(9.0_f64);
    let t20406 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12529 * t13300 * t19314;
    let t20409 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t5078 * t6364;
    (t20394, t20397, t20400, t20403, t20406, t20409)
}
