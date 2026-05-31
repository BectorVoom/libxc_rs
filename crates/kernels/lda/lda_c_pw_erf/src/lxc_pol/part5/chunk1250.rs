//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1250/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1250<F: Float>(t544: F, t7466: F, t2072: F, t6601: F, t511: F, t7522: F, t172: F, t184: F, t7659: F, t496: F, t14044: F, t256: F, t652: F, t8032: F) -> (F, F, F, F, F, F) {
    let t22403 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t7466 * t544;
    let t22405 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6601 * t2072;
    let t22407 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t511 * t7522;
    let t22409 = t172 * t7659 * t184;
    let t22411 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t22409 * t496;
    let t22412 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t14044;
    let t22418 = t8032 * t652 * t256;
    (t22403, t22405, t22407, t22411, t22412, t22418)
}
