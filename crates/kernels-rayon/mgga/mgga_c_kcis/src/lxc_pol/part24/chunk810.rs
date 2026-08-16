//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 810/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk810(t13238: f64, t13242: f64, t3622: f64, t5341: f64, t13270: f64, t13277: f64, t13301: f64, t13305: f64, t13307: f64, t1239: f64, t5272: f64, t11081: f64, t5306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15157 = 0.61905925925925925925e-2_f64 * t13238;
    let t15158 = 0.25794135802469135802e-2_f64 * t13242;
    let t15171 = t5341 * t3622;
    let t15179 = 0.10317654320987654321e-2_f64 * t13270;
    let t15182 = 0.23214722222222222222e-2_f64 * t13277;
    let t15189 = 0.10317654320987654321e-2_f64 * t13301;
    let t15191 = 0.61905925925925925925e-2_f64 * t13305;
    let t15192 = 0.15476481481481481481e-2_f64 * t13307;
    let t15198 = t5272 * t1239;
    let t15213 = t11081 * t5306;
    (t15157, t15158, t15171, t15179, t15182, t15189, t15191, t15192, t15198, t15213)
}
