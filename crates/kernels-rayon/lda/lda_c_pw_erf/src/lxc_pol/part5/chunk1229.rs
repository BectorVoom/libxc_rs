//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1229/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1229(t1318: f64, t3899: f64, t7569: f64, t4763: f64, t6894: f64, t2143: f64, t6198: f64, t6190: f64, t20027: f64, t571: f64, t574: f64, t575: f64) -> (f64, f64, f64, f64, f64) {
    let t22156 = t1318 * t3899 * t7569;
    let t22157 = 8.0_f64 / 15.0_f64 * t22156;
    let t22158 = t4763 * t6894;
    let t22159 = 16.0_f64 / 15.0_f64 * t22158;
    let t22160 = t6198 * t2143;
    let t22161 = 8.0_f64 / 45.0_f64 * t22160;
    let t22163 = 4.0_f64 / 5.0_f64 * t4763 * t6190;
    let t22167 = 4.0_f64 / 45.0_f64 * t571 * t574 * t575 * t20027;
    (t22157, t22159, t22161, t22163, t22167)
}
