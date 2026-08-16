//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 720/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk720<F: Float>(t593: F, t743: F, t352: F, t4515: F, t4506: F, t1484: F, t581: F, t1351: F) -> (F, F, F, F, F, F) {
    let t4516 = t743 * t593;
    let t4517 = t4516 * t352;
    let t4518 = t4515 * t4517;
    let t4520 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4506 * t4518;
    let t4521 = t1484 * t581;
    let t4522 = t4521 * t1351;
    (t4516, t4517, t4518, t4520, t4521, t4522)
}
