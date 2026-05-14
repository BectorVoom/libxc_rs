//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1122/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1122<F: Float>(t2783: F, t9334: F, t10919: F, t1850: F, t17351: F, t17664: F, t20705: F, t20905: F, t21093: F, t21165: F, t248: F, t25633: F, t25636: F, t30197: F, t30200: F, t30203: F, t30205: F, t30208: F, t30211: F, t30213: F, t30216: F, t30219: F, t30284: F, t30287: F, t30377: F, t3577: F, t3604: F, t5845: F, t5871: F, t702: F, t721: F, t7247: F, t7308: F, t9419: F, t9446: F) -> (F, F, F) {
    let t30385 = 3.0 * t9334 * t2783;
    let t30387 = 1.0 * t1850 * t10919;
    let t30410 = 18.0 * t20905 * t9419 + 0.10526802520742363173e2 * t21093 * t9446 - 0.310907e-1 * (t17664 - 0.53272592592592592592e-1 * t17351 - 0.15981777777777777777e0 * t20705 + t21165 + 0.68493333333333333332e-1 * t25633 - 0.51369999999999999999e-1 * t25636 - 0.17123333333333333333e-1 * t30284 + 0.5137e-1 * t30287) * t248 + 0.30762056574649219973e4 * t5845 * t3604 * t7308 * t721 - 0.19751673498613801407e-1 * t30377 + t30197 + 0.62071215503128080361e4 * t5871 * t3577 * t7247 * t702 - t30200 - t30203 + t30205 + t30208 + t30211 - t30213 - t30216 - t30219;
    (t30385, t30387, t30410)
}
