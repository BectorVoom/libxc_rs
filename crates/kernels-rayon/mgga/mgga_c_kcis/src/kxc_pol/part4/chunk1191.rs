//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1191/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1191(t13301: f64, t13305: f64, t13307: f64, t1239: f64, t5272: f64, t11081: f64, t5306: f64, t3514: f64, t2880: f64, t421: f64, t4581: f64, t9959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15189 = 0.10317654320987654321e-2_f64 * t13301;
    let t15191 = 0.61905925925925925925e-2_f64 * t13305;
    let t15192 = 0.15476481481481481481e-2_f64 * t13307;
    let t15198 = t5272 * t1239;
    let t15213 = t11081 * t5306;
    let t15215 = t3514 * t15213 / 864.0_f64;
    let t15216 = t2880 * t421;
    let t15217 = t15216 * t4581;
    let t15219 = t3514 * t15217 / 432.0_f64;
    let t15220 = t9959 * t421;
    (t15189, t15191, t15192, t15198, t15215, t15219, t15220)
}
