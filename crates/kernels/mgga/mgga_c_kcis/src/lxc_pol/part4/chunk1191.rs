//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1191/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1191<F: Float>(t13301: F, t13305: F, t13307: F, t1239: F, t5272: F, t11081: F, t5306: F, t3514: F, t2880: F, t421: F, t4581: F, t9959: F) -> (F, F, F, F, F, F, F) {
    let t15189 = F::new(0.10317654320987654321e-2) * t13301;
    let t15191 = F::new(0.61905925925925925925e-2) * t13305;
    let t15192 = F::new(0.15476481481481481481e-2) * t13307;
    let t15198 = t5272 * t1239;
    let t15213 = t11081 * t5306;
    let t15215 = t3514 * t15213 / F::new(864.0);
    let t15216 = t2880 * t421;
    let t15217 = t15216 * t4581;
    let t15219 = t3514 * t15217 / F::new(432.0);
    let t15220 = t9959 * t421;
    (t15189, t15191, t15192, t15198, t15215, t15219, t15220)
}
