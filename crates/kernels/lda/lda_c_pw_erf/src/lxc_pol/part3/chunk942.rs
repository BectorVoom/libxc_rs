//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 942/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk942<F: Float>(t12460: F, t10011: F, t5143: F, t5148: F, t12427: F, t12432: F, t12435: F, t12438: F, t12442: F, t12444: F, t12449: F, t12453: F, t12456: F, t12459: F, t1278: F, t3965: F, t3967: F, t5136: F) -> (F, F, F, F, F) {
    let t12461 = 32.0 / 45.0 * t12460;
    let t12462 = t10011 * t5143;
    let t12463 = 64.0 / 45.0 * t12462;
    let t12464 = t10011 * t5148;
    let t12465 = 32.0 / 27.0 * t12464;
    let t12466 = t12427 + t12432 + t12435 + t12438 + t12442 + t12444 + t12449 + t12453 + t12456 + t12459 - t12461 - t12463 + t12465;
    let t12474 = 8.0 / 15.0 * t3965 * t3967 * t5136 * t1278;
    (t12461, t12463, t12465, t12466, t12474)
}
