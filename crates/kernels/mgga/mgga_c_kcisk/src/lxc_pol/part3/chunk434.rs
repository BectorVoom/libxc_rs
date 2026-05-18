//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 434/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk434<F: Float>(t264: F, t1097: F, t1100: F, t1099: F, t281: F, t259: F, t1128: F, t278: F, t2925: F, t67: F, t10: F, t1102: F, t119: F, t142: F, t260: F, t261: F) -> (F, F, F, F, F, F, F, F, F) {
    let t265 = t264 < -F::new(0.66725e-1);
    let t3368 = t1097 * t1100;
    let t3372 = F::new(1.0) / t1099 / t281;
    let t3373 = t259 * t3372;
    let t3374 = t1128 * t1128;
    let t3375 = t278 * t278;
    let t3376 = F::new(1.0) / t3375;
    let t3377 = t3374 * t3376;
    let t3380 = t67 * t2925;
    let t3391 = piecewise3::<f64>(t265, F::new(0.0), F::new(10.0) / F::new(9.0) * t260 * t3380 * t10 - F::new(20.0) / F::new(27.0) * t260 * t1102 * t142 + F::new(40.0) / F::new(81.0) * t260 * t261 * t119);
    (t3368, t3372, t3373, t3374, t3375, t3376, t3377, t3380, t3391)
}
