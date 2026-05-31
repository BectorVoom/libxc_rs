//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 775/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk775<F: Float>(t10: F, t127: F, t1832: F, t1852: F, t411: F, t426: F, t5596: F, t7109: F, t7112: F, t7115: F, t7116: F, t7123: F, t7128: F, t7129: F, t7133: F, t7137: F, t7164: F) -> F {
    let t7166 = t7109 + t7112 + t7115 - F::cast_from(29.3808_f64) * t127 * t7116 * t411 + F::cast_from(11.75232_f64) * t127 * t1852 * t1832 - F::cast_from(1.46904_f64) * t127 * t7123 - t7128 - F::cast_from(6.0_f64) * t426 * t10 * t7129 + F::cast_from(3.0_f64) * t426 * t10 * t7133 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t7137 - t5596 + t7164;
    t7166
}
