//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1030/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1030<F: Float>(t13238: F, t13242: F, t13270: F, t13277: F, t13301: F, t13305: F, t13307: F, t1239: F, t5272: F, t11081: F, t5306: F, t3514: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15157 = F::cast_from(0.61905925925925925925e-2_f64) * t13238;
    let t15158 = F::cast_from(0.25794135802469135802e-2_f64) * t13242;
    let t15179 = F::cast_from(0.10317654320987654321e-2_f64) * t13270;
    let t15182 = F::cast_from(0.23214722222222222222e-2_f64) * t13277;
    let t15189 = F::cast_from(0.10317654320987654321e-2_f64) * t13301;
    let t15191 = F::cast_from(0.61905925925925925925e-2_f64) * t13305;
    let t15192 = F::cast_from(0.15476481481481481481e-2_f64) * t13307;
    let t15198 = t5272 * t1239;
    let t15213 = t11081 * t5306;
    let t15215 = t3514 * t15213 / F::new(864.0);
    (t15157, t15158, t15179, t15182, t15189, t15191, t15192, t15198, t15215)
}
