//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1093/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1093<F: Float>(t39445: F, t10772: F, t10810: F, t2568: F, t10768: F, t8129: F, t2604: F, t625: F, t37637: F, t24906: F, t37943: F, t37945: F) -> (F, F, F, F, F, F) {
    let t39446 = F::new(0.47609969197673950972e-2) * t39445;
    let t39458 = t10772 * t10810 * t2568;
    let t39459 = F::new(0.69345773920434148506e0) * t39458;
    let t39464 = t10768 * t8129;
    let t39469 = t2604 * t625;
    let t39470 = t37637 * t39469;
    let t39482 = t37943 * t37945 * t24906;
    (t39446, t39459, t39464, t39469, t39470, t39482)
}
