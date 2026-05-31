//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1054/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1054<F: Float>(t3850: F, t4488: F, t4490: F, t12321: F, t3403: F, t806: F, t4561: F, t565: F, t1522: F, t184: F, t1958: F, t221: F) -> (F, F, F, F) {
    let t12351 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4488 * t4490 * t3850;
    let t12355 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4488 * t12321 * t806 * t3403;
    let t12356 = t565 * t4561;
    let t12357 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12356;
    let t12361 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1522 * t1958 * t184 * t221;
    (t12351, t12355, t12357, t12361)
}
