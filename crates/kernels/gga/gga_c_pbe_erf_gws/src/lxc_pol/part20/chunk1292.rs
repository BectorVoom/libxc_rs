//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1292/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1292<F: Float>(t11572: F, t50998: F, t51066: F, t1144: F, t14402: F, t4386: F, t1161: F, t353: F, t53614: F, t859: F, t14418: F, t11450: F, t13917: F, t51544: F) -> (F, F, F, F, F) {
    let t56442 = t50998 * t51066 * t11572;
    let t56445 = t4386 * t1144 * t14402;
    let t56452 = t859 * t353 * t53614 * t1161;
    let t56456 = t859 * t1144 * t14418;
    let t56460 = t13917 * t51544 * t11450;
    (t56442, t56445, t56452, t56456, t56460)
}
