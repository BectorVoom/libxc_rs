//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1120/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1120<F: Float>(t13105: F, t5155: F, t954: F, t3974: F, t5166: F, t951: F, t11914: F, t3704: F, t3973: F, t34: F, t549: F, t352: F) -> (F, F, F, F, F, F, F, F) {
    let t13106 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t13105;
    let t13107 = t5155 * t954;
    let t13110 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3974 * t5166 * t13107;
    let t13111 = t5155 * t951;
    let t13114 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3974 * t11914 * t13111;
    let t13115 = t3973 * t3704;
    let t13116 = t34 * t549;
    let t13117 = t13116 * t352;
    (t13106, t13107, t13110, t13111, t13114, t13115, t13116, t13117)
}
