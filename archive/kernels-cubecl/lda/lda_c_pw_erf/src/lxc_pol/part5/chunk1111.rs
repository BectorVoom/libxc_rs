//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1111/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1111<F: Float>(t20701: F, t3974: F, t5160: F, t5166: F, t18188: F, t2026: F, t3965: F, t2334: F, t833: F, t352: F, t13829: F, t4506: F) -> (F, F, F, F, F, F) {
    let t20704 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3974 * t5160 * t20701;
    let t20707 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3974 * t5166 * t20701;
    let t20710 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t18188 * t2026;
    let t20711 = t2334 * t833;
    let t20712 = t20711 * t352;
    let t20715 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t4506 * t13829 * t20712;
    (t20704, t20707, t20710, t20711, t20712, t20715)
}
