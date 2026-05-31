//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1172/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1172<F: Float>(t17107: F, t17109: F, t17112: F, t17114: F, t2134: F, t2407: F, t17117: F, t12475: F, t6442: F, t6762: F, t2325: F, t806: F) -> (F, F, F, F, F, F, F, F) {
    let t21387 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17107;
    let t21388 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17109;
    let t21389 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17112;
    let t21390 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17114;
    let t21391 = t2407 * t2134;
    let t21392 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t21391;
    let t21393 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t17117;
    let t21396 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t12475 * t6762 * t6442;
    let t21397 = t2325 * t806;
    (t21387, t21388, t21389, t21390, t21392, t21393, t21396, t21397)
}
