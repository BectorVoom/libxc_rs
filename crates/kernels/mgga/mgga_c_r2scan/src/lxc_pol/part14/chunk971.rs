//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 971/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk971<F: Float>(t11351: F, t3560: F, t885: F, t10656: F, t10923: F, t10932: F, t10944: F, t10947: F, t10678: F, t10685: F, t10692: F, t10695: F, t10937: F, t11205: F, t11328: F) -> (F, F, F, F, F, F, F, F) {
    let t11352 = t11351 / F::cast_from(4.0_f64);
    let t11353 = t3560 * t885;
    let t11354 = F::cast_from(2.0_f64) * t11353;
    let t11357 = F::cast_from(0.30487649791575028312e-3_f64) * t10656;
    let t11364 = F::cast_from(0.30487649791575028312e-3_f64) * t10923;
    let t11365 = F::cast_from(0.86737941314158990616e-4_f64) * t10932;
    let t11367 = F::cast_from(0.60975299583150056624e-3_f64) * t10944;
    let t11368 = F::cast_from(0.162600798888400151e-2_f64) * t10947;
    let t11369 = t11205 - F::cast_from(0.20496175532535769482e-3_f64) * t10678 + F::cast_from(0.1440846329149835838e-2_f64) * t10685 + t11328 - F::cast_from(0.72042316457491791901e-3_f64) * t10692 - F::cast_from(0.1440846329149835838e-2_f64) * t10695 + t11364 - t11365 + F::cast_from(0.3842256877732895568e-2_f64) * t10937 + t11367 + t11368;
    (t11352, t11354, t11357, t11364, t11365, t11367, t11368, t11369)
}
