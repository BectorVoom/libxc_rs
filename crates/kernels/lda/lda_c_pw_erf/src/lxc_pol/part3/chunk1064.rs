//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1064/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1064<F: Float>(t12464: F, t12427: F, t12432: F, t12435: F, t12438: F, t12442: F, t12444: F, t12449: F, t12453: F, t12456: F, t12459: F, t12461: F, t12463: F) -> (F, F) {
    let t12465 = F::new(32.0) / F::new(27.0) * t12464;
    let t12466 = t12427 + t12432 + t12435 + t12438 + t12442 + t12444 + t12449 + t12453 + t12456 + t12459 - t12461 - t12463 + t12465;
    (t12465, t12466)
}
