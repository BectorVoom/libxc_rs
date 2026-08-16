//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1190/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1190(t3616: f64, t5281: f64, t3622: f64, t5341: f64, t1267: f64, t13270: f64, t13277: f64, t11151: f64, t13247: f64, t13252: f64, t13254: f64, t13258: f64, t13263: f64, t13268: f64, t13275: f64, t13282: f64, t13286: f64, t13290: f64, t13294: f64, t13298: f64, t3644: f64, t5282: f64, t9529: f64, t9536: f64, t9552: f64) -> (f64, f64, f64) {
    let t15168 = t5281 * t3616;
    let t15171 = t5341 * t3622;
    let t15172 = t15171 * t1267;
    let t15179 = 0.10317654320987654321e-2_f64 * t13270;
    let t15182 = 0.23214722222222222222e-2_f64 * t13277;
    let t15188 = 0.77382407407407407406e-3_f64 * t9529 - 0.25794135802469135802e-3_f64 * t9536 + 0.61905925925925925925e-2_f64 * t13247 + 0.30952962962962962962e-2_f64 * t13252 - 0.23214722222222222222e-2_f64 * t13254 - 0.23214722222222222222e-2_f64 * t13258 + 0.890445125e-2_f64 * t3644 * t15168 + 0.178089025e-1_f64 * t3644 * t15172 + 0.178089025e-1_f64 * t11151 * t5282 + 0.38691203703703703703e-3_f64 * t13263 + 0.23214722222222222222e-2_f64 * t13268 + t15179 - 0.51588271604938271604e-3_f64 * t9552 + 0.92858888888888888886e-2_f64 * t13275 - t15182 - 0.11607361111111111111e-2_f64 * t13282 - 0.11607361111111111111e-2_f64 * t13286 - 0.19345601851851851852e-2_f64 * t13290 + 0.77382407407407407407e-3_f64 * t13294 + 0.12897067901234567901e-2_f64 * t13298;
    (t15168, t15172, t15188)
}
