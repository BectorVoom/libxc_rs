//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 531/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk531<F: Float>(t1172: F, t1530: F, t301: F, t944: F, t396: F, t980: F, t409: F, t360: F, t372: F, t177: F, t414: F, t377: F, t973: F) -> (F, F, F, F, F, F, F, F) {
    let t3462 = t1530 * t1172;
    let t3463 = t944 * t301;
    let t3476 = t980 * t396;
    let t3477 = t3476 * t409;
    let t3539 = t944 * t360;
    let t3544 = t944 * t372;
    let t3551 = F::cast_from(0.30011812682648815881e-2_f64) * t980 * t414 * t177;
    let t3556 = F::cast_from(0.17006693853500995666e-1_f64) * t377 * t973 * t177;
    (t3462, t3463, t3476, t3477, t3539, t3544, t3551, t3556)
}
