//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 877/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk877<F: Float>(t4360: F, t7730: F, t1063: F, t1052: F, t1076: F, t1095: F, t2305: F, t2341: F, t2355: F, t3142: F, t3335: F, t3342: F, t4283: F, t4335: F, t8096: F, t8101: F, t9128: F, t9131: F, t9134: F, t9136: F, t98: F) -> (F, F) {
    let t9150 = t4360 * t7730;
    let t9151 = t1063 * t9150;
    let t9153 = t4283 + t2305 * t4335 / F::new(36.0) - t9128 * t98 / F::new(6.0) - t9131 / F::new(27.0) - t9134 / F::new(6.0) + t9136 / F::new(9.0) + F::cast_from(0.10237773105191754_f64) * t3335 + F::cast_from(0.06825182070127836_f64) * t3342 + t1076 * t8096 / F::new(6.0) + t1076 * t8101 / F::new(6.0) - t1095 * t2341 / F::new(6.0) + t1052 * t8096 / F::new(6.0) + t2355 * t3142 / F::new(6.0) - t9151 / F::new(6.0);
    (t9150, t9153)
}
