//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1981/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1981<F: Float>(t5611: F, t852: F, t17100: F, t225: F, t17087: F, t17060: F, t17095: F, t17098: F, t18287: F, t1176: F, t1714: F, t19256: F) -> (F, F, F, F, F, F, F, F, F) {
    let t59331 = t852 * t5611;
    let t59466 = t17100 * t225;
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t64595 = t18287 * t225;
    let t64825 = t1176 * t1714;
    let t65203 = t19256 * t225;
    (t59331, t59466, t59498, t59503, t59519, t59537, t64595, t64825, t65203)
}
