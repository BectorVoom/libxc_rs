//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 515/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk515<F: Float>(t396: F, t980: F, t409: F, t360: F, t944: F, t372: F, t177: F, t414: F, t377: F, t973: F, t1963: F, t22: F, t161: F, t151: F, t334: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3476 = t980 * t396;
    let t3477 = t3476 * t409;
    let t3539 = t944 * t360;
    let t3544 = t944 * t372;
    let t3551 = 0.30011812682648815881e-2 * t980 * t414 * t177;
    let t3556 = 0.17006693853500995666e-1 * t377 * t973 * t177;
    let t3558 = 1.0 / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = 0.37792653007779990369e-1 * t151 * t3559 * t177;
    let t3570 = t986 * t334;
    (t3476, t3477, t3539, t3544, t3551, t3556, t3558, t3562, t3570)
}
