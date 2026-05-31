//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 570/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk570<F: Float>(t3497: F, t3523: F, t1161: F, t1170: F, t1180: F, t1189: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3444: F, t3447: F, t3452: F, t3454: F, t3472: F, t3477: F, t3480: F, t3489: F, t3491: F, t3496: F, t3498: F, t3516: F, t3521: F, t435: F) -> (F, F) {
    let t3524 = t3497 * t3523;
    let t3527 = -F::cast_from(0.310907e-1_f64) * t3444 * t435 + F::cast_from(2.0_f64) * t3447 * t1170 - F::cast_from(2.0_f64) * t3452 * t3454 + F::cast_from(1.0_f64) * t1161 * t3472 + F::cast_from(0.32163958997385070134e2_f64) * t3477 * t3480 + t3378 - t3381 + t3388 - t3430 - t3438 - F::cast_from(0.19751673498613801407e-1_f64) * t3489 + F::cast_from(0.11696447245269292414e1_f64) * t3491 * t1189 - F::cast_from(0.11696447245269292414e1_f64) * t3496 * t3498 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t3516 + F::cast_from(0.17315859105681463759e2_f64) * t3521 * t3524;
    (t3524, t3527)
}
