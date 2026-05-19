//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 124/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk124<F: Float>(t390: F, t185: F, t2: F, t387: F, t22: F, t23: F, t6: F, t8: F, t388: F, t31: F) -> (F, F, F, F, F, F, F, F) {
    let t391 = F::cast_from(0.29896666666666666667e0_f64) * t390;
    let t392 = t185 * t2;
    let t393 = t392 * t387;
    let t394 = F::new(0.1023875e0) * t393;
    let t398 = t22 * t6 / t23 / t8;
    let t399 = F::cast_from(0.82156666666666666667e-1_f64) * t398;
    let t400 = -F::new(0.632975e0) * t388 - t391 - t394 - t399;
    let t401 = F::new(1.0) / t31;
    (t391, t392, t393, t394, t398, t399, t400, t401)
}
