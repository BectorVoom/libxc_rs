//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1189/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1189(t1844: f64, t3643: f64, t13238: f64, t13242: f64, t13219: f64, t13222: f64, t13225: f64, t13228: f64, t13231: f64, t3617: f64, t3623: f64, t5345: f64, t9522: f64, t9524: f64) -> (f64, f64) {
    let t15134 = t1844 * t3643;
    let t15157 = 0.61905925925925925925e-2_f64 * t13238;
    let t15158 = 0.25794135802469135802e-2_f64 * t13242;
    let t15159 = 0.77382407407407407407e-3_f64 * t13219 - 0.41270617283950617284e-2_f64 * t13222 + 0.12381185185185185185e-1_f64 * t13225 - 0.10317654320987654321e-1_f64 * t13228 + 0.92858888888888888886e-2_f64 * t13231 + 0.77382407407407407407e-3_f64 * t9522 - 0.23214722222222222222e-2_f64 * t9524 - 0.66725e-1_f64 * t5345 * t3617 + 0.66725e-1_f64 * t5345 * t3623 - t15157 + t15158;
    (t15134, t15159)
}
