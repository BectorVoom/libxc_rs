//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1131/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1131<F: Float>(t3802: F, t4749: F, t519: F, t5260: F, t13080: F, t1318: F, t4784: F, t11697: F, t1991: F, t11766: F, t4829: F, t1472: F, t5302: F) -> (F, F, F, F, F, F) {
    let t13238 = t519 * t3802 * t4749;
    let t13239 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13238;
    let t13241 = t519 * t3802 * t5260;
    let t13242 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13241;
    let t13244 = t1318 * t13080 * t4784;
    let t13245 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13244;
    let t13248 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t519 * t1991 * t11697;
    let t13251 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t519 * t4829 * t11766;
    let t13252 = t1472 * t5302;
    (t13239, t13242, t13245, t13248, t13251, t13252)
}
