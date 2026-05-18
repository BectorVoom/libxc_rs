//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 810/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk810<F: Float>(t13238: F, t13242: F, t3622: F, t5341: F, t13270: F, t13277: F, t13301: F, t13305: F, t13307: F, t1239: F, t5272: F, t11081: F, t5306: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15157 = F::new(0.61905925925925925925e-2) * t13238;
    let t15158 = F::new(0.25794135802469135802e-2) * t13242;
    let t15171 = t5341 * t3622;
    let t15179 = F::new(0.10317654320987654321e-2) * t13270;
    let t15182 = F::new(0.23214722222222222222e-2) * t13277;
    let t15189 = F::new(0.10317654320987654321e-2) * t13301;
    let t15191 = F::new(0.61905925925925925925e-2) * t13305;
    let t15192 = F::new(0.15476481481481481481e-2) * t13307;
    let t15198 = t5272 * t1239;
    let t15213 = t11081 * t5306;
    (t15157, t15158, t15171, t15179, t15182, t15189, t15191, t15192, t15198, t15213)
}
