//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 611/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk611<F: Float>(t322: F, t1074: F, t829: F, t1300: F, t327: F, t3370: F, t3373: F, t834: F, t330: F, t1079: F, t837: F, t3369: F) -> (F, F, F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t3376 = t1074 * t829;
    let t3381 = -F::new(0.64e0) * t3370 * t327 - F::new(0.128e1) * t3373 * t829 - F::new(0.128e1) * t1300 * t3376 - F::new(0.64e0) * t834 * t3370;
    let t3382 = t3381 * t330;
    let t3383 = t1079 * t837;
    let t3384 = t3383 * t330;
    let t3386 = piecewise3::<F>(t332, F::new(0.0), t3369);
    (t3381, t3382, t3384, t3386)
}
