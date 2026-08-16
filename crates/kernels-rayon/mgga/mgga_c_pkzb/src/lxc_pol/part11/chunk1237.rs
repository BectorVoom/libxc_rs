//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1237/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1237(t2783: f64, t9334: f64, t10919: f64, t1850: f64, t17351: f64, t17664: f64, t20705: f64, t20905: f64, t21093: f64, t21165: f64, t248: f64, t25633: f64, t25636: f64, t30197: f64, t30200: f64, t30203: f64, t30205: f64, t30208: f64, t30211: f64, t30213: f64, t30216: f64, t30219: f64, t30284: f64, t30287: f64, t30377: f64, t3577: f64, t3604: f64, t5845: f64, t5871: f64, t702: f64, t721: f64, t7247: f64, t7308: f64, t9419: f64, t9446: f64) -> (f64, f64, f64) {
    let t30385 = 3.0_f64 * t9334 * t2783;
    let t30387 = 1.0_f64 * t1850 * t10919;
    let t30410 = 18.0_f64 * t20905 * t9419 + 0.10526802520742363173e2_f64 * t21093 * t9446 - 0.310907e-1_f64 * (t17664 - 0.53272592592592592592e-1_f64 * t17351 - 0.15981777777777777777e0_f64 * t20705 + t21165 + 0.68493333333333333332e-1_f64 * t25633 - 0.51369999999999999999e-1_f64 * t25636 - 0.17123333333333333333e-1_f64 * t30284 + 0.5137e-1_f64 * t30287) * t248 + 0.30762056574649219973e4_f64 * t5845 * t3604 * t7308 * t721 - 0.19751673498613801407e-1_f64 * t30377 + t30197 + 0.62071215503128080361e4_f64 * t5871 * t3577 * t7247 * t702 - t30200 - t30203 + t30205 + t30208 + t30211 - t30213 - t30216 - t30219;
    (t30385, t30387, t30410)
}
