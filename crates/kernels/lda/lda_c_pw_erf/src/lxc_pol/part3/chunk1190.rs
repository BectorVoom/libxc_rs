//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1190/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1190<F: Float>(t3850: F, t3965: F, t4479: F, t4500: F, t784: F, t3403: F, t3412: F, t6762: F, t10027: F, t4476: F, t3824: F, t3974: F, t4475: F) -> (F, F, F, F, F) {
    let t14013 = F::new(8.0) / F::new(15.0) * t3965 * t4479 * t3850;
    let t14014 = t4500 * t784;
    let t14017 = F::new(8.0) / F::new(9.0) * t3965 * t14014 * t3403;
    let t14020 = F::new(16.0) / F::new(15.0) * t3965 * t6762 * t3412;
    let t14022 = F::new(16.0) / F::new(15.0) * t10027 * t4476;
    let t14025 = F::new(8.0) / F::new(15.0) * t3974 * t4475 * t3824;
    (t14013, t14017, t14020, t14022, t14025)
}
