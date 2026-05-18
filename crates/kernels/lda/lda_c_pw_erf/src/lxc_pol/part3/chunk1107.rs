//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1107/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1107<F: Float>(t10164: F, t10167: F, t4647: F, t515: F, t3872: F, t3974: F, t4475: F, t4489: F, t784: F, t3807: F, t3965: F, t3811: F, t4479: F) -> (F, F, F, F, F, F) {
    let t12949 = F::new(16.0) / F::new(45.0) * t10164;
    let t12950 = F::new(32.0) / F::new(405.0) * t10167;
    let t12951 = t4647 * t515;
    let t12952 = F::new(4.0) / F::new(15.0) * t12951;
    let t12955 = F::new(16.0) / F::new(15.0) * t3974 * t4475 * t3872;
    let t12956 = t4489 * t784;
    let t12959 = F::new(16.0) / F::new(15.0) * t3965 * t12956 * t3807;
    let t12962 = F::new(16.0) / F::new(15.0) * t3965 * t4479 * t3811;
    (t12949, t12950, t12952, t12955, t12959, t12962)
}
