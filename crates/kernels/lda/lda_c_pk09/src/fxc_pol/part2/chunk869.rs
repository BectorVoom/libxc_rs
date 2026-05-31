//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 869/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk869<F: Float>(t1011: F, t2355: F, t3173: F, t3177: F, t3191: F, t4168: F, t4170: F, t4177: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F) -> F {
    let t9032 = t4168 - t4170 - t2355 * t1011 / F::cast_from(6.0_f64) - t4177 + F::cast_from(0.06825182070127836_f64) * t7801 + F::cast_from(0.10237773105191754_f64) * t7805 + F::cast_from(0.10237773105191754_f64) * t7809 + F::cast_from(0.10237773105191754_f64) * t7811 + F::cast_from(0.10237773105191754_f64) * t7814 + F::cast_from(0.10237773105191754_f64) * t7817 + F::cast_from(0.10237773105191754_f64) * t7834 + F::cast_from(0.02466859483068398_f64) * t3173 - F::cast_from(0.02466859483068398_f64) * t3177 + F::cast_from(0.02466859483068398_f64) * t3191;
    t9032
}
