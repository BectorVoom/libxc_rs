//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 958/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk958<F: Float>(t11529: F, t2847: F, t797: F, t3275: F, t3276: F, t3696: F, t860: F, t1044: F, t3424: F, t3685: F, t885: F, t4176: F, t986: F) -> (F, F, F, F, F, F, F, F) {
    let t11530 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11529;
    let t11531 = t797 * t2847;
    let t11533 = t3275 * t3276 * t11531;
    let t11534 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11533;
    let t11535 = t860 * t3696;
    let t11537 = t3424 * t1044;
    let t11538 = t3685 * t885;
    let t11539 = t4176 * t986;
    (t11530, t11531, t11533, t11534, t11535, t11537, t11538, t11539)
}
