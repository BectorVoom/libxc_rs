//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 688/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk688<F: Float>(t1620: F, t226: F, t603: F, t695: F, t1612: F, t230: F, t598: F, t610: F, t225: F, t2853: F, t611: F, t1621: F) -> (F, F, F, F, F, F, F) {
    let t4215 = F::new(4.0) * t226 * t1620;
    let t4217 = F::cast_from(0.0011033703703703704_f64) * t695 * t603;
    let t4218 = t1612 * t230;
    let t4220 = t598 * t610;
    let t4222 = t2853 * t225;
    let t4225 = t1612 * t611;
    let t4227 = t598 * t1621;
    (t4215, t4217, t4218, t4220, t4222, t4225, t4227)
}
