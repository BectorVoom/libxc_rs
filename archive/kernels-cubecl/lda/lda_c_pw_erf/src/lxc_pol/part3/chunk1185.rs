//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1185/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1185<F: Float>(t1351: F, t4574: F, t13111: F, t3974: F, t3975: F, t13813: F, t4506: F, t11909: F, t10027: F, t5152: F, t2104: F, t5175: F) -> (F, F, F, F, F) {
    let t13962 = t4574 * t1351;
    let t13965 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t3974 * t13962 * t13111;
    let t13966 = t3975 * t1351;
    let t13969 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4506 * t13966 * t13813;
    let t13972 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4506 * t13962 * t11909;
    let t13974 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10027 * t5152;
    let t13975 = t2104 * t5175;
    (t13965, t13969, t13972, t13974, t13975)
}
