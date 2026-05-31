//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1280/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1280<F: Float>(t13115: F, t34: F, t5166: F, t743: F, t811: F, t21420: F, t3974: F, t4522: F, t13797: F, t22240: F, t4506: F, t16863: F, t2006: F, t3965: F) -> (F, F, F, F) {
    let t22931 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t13115 * t5166 * t743 * t811 * t34;
    let t22934 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3974 * t4522 * t21420;
    let t22937 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4506 * t13797 * t22240;
    let t22940 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t16863 * t2006;
    (t22931, t22934, t22937, t22940)
}
