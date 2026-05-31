//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 916/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk916<F: Float>(t4233: F, t598: F, t226: F, t4606: F, t5021: F, t7: F, t1458: F, t155: F, t4049: F, t581: F, t1620: F, t4232: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t10278 = t598 * t4233;
    let t10286 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t226 * (-F::cast_from(4.277777777777778_f64) * t4606 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t5021) * pi * t7;
    let t10313 = t155 * t1458;
    let t10379 = t4049 * t581;
    let t10409 = t598 * t1620;
    let t10412 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t226 * t4232;
    (t10278, t10286, t10313, t10379, t10409, t10412)
}
