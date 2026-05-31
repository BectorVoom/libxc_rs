//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1126/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1126<F: Float>(t1124: F, t188: F, t1325: F, t4958: F, t2171: F, t3735: F, t3739: F, t3787: F, t4952: F, t1440: F, t3675: F, t3677: F, t784: F) -> (F, F, F, F, F) {
    let t13172 = t1124 * t188;
    let t13174 = t1325 * t13172 * t4958;
    let t13175 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13174;
    let t13176 = t2171 * t3735;
    let t13177 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13176;
    let t13179 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2171 * t3739;
    let t13181 = t1325 * t3787 * t4952;
    let t13182 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13181;
    let t13187 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1325 * t1440 * t3675 * t784 * t3677;
    (t13175, t13177, t13179, t13182, t13187)
}
