//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1060/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1060<F: Float>(t3622: F, t5341: F, t1267: F, t13270: F, t13277: F, t11151: F, t13247: F, t13252: F, t13254: F, t13258: F, t13263: F, t13268: F, t13275: F, t13282: F, t13286: F, t13290: F, t13294: F, t13298: F, t15168: F, t3644: F, t5282: F, t9529: F, t9536: F, t9552: F) -> (F, F) {
    let t15171 = t5341 * t3622;
    let t15172 = t15171 * t1267;
    let t15179 = 0.10317654320987654321e-2 * t13270;
    let t15182 = 0.23214722222222222222e-2 * t13277;
    let t15188 = 0.77382407407407407406e-3 * t9529 - 0.25794135802469135802e-3 * t9536 + 0.61905925925925925925e-2 * t13247 + 0.30952962962962962962e-2 * t13252 - 0.23214722222222222222e-2 * t13254 - 0.23214722222222222222e-2 * t13258 + 0.890445125e-2 * t3644 * t15168 + 0.178089025e-1 * t3644 * t15172 + 0.178089025e-1 * t11151 * t5282 + 0.38691203703703703703e-3 * t13263 + 0.23214722222222222222e-2 * t13268 + t15179 - 0.51588271604938271604e-3 * t9552 + 0.92858888888888888886e-2 * t13275 - t15182 - 0.11607361111111111111e-2 * t13282 - 0.11607361111111111111e-2 * t13286 - 0.19345601851851851852e-2 * t13290 + 0.77382407407407407407e-3 * t13294 + 0.12897067901234567901e-2 * t13298;
    (t15172, t15188)
}
