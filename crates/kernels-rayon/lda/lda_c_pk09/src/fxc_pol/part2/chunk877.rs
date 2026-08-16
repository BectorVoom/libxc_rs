//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 877/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk877(t4360: f64, t7730: f64, t1063: f64, t1052: f64, t1076: f64, t1095: f64, t2305: f64, t2341: f64, t2355: f64, t3142: f64, t3335: f64, t3342: f64, t4283: f64, t4335: f64, t8096: f64, t8101: f64, t9128: f64, t9131: f64, t9134: f64, t9136: f64, t98: f64) -> (f64, f64) {
    let t9150 = t4360 * t7730;
    let t9151 = t1063 * t9150;
    let t9153 = t4283 + t2305 * t4335 / 36.0_f64 - t9128 * t98 / 6.0_f64 - t9131 / 27.0_f64 - t9134 / 6.0_f64 + t9136 / 9.0_f64 + 0.10237773105191754_f64 * t3335 + 0.06825182070127836_f64 * t3342 + t1076 * t8096 / 6.0_f64 + t1076 * t8101 / 6.0_f64 - t1095 * t2341 / 6.0_f64 + t1052 * t8096 / 6.0_f64 + t2355 * t3142 / 6.0_f64 - t9151 / 6.0_f64;
    (t9150, t9153)
}
