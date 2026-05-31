//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 782/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk782<F: Float>(t1958: F, t202: F, t184: F, t551: F, t172: F, t1980: F, t496: F, t1245: F, t806: F, t940: F, t3402: F, t519: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5210 = t202 * t1958;
    let t5211 = t5210 * t184;
    let t5213 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5211 * t551;
    let t5214 = t172 * t1980;
    let t5215 = t5214 * t184;
    let t5217 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5215 * t496;
    let t5220 = t806 * t1245;
    let t5221 = t5220 * t940;
    let t5222 = t3402 * t5221;
    let t5224 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t519 * t5222;
    (t5210, t5211, t5213, t5214, t5215, t5217, t5220, t5221, t5222, t5224)
}
