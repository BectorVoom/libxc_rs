//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 839/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk839<F: Float>(t3491: F, t631: F, t184: F, t221: F, t7041: F, t3488: F, t583: F, t1791: F, t3553: F, t661: F, t1621: F, t639: F, t1630: F, t3499: F, t4993: F, t3479: F, t663: F) -> (F, F, F, F, F, F, F) {
    let t10287 = t3491 * t631;
    let t10288 = t10287 * t184;
    let t10290 = 4.0 / 15.0 * t10288 * t221;
    let t10291 = 32.0 / 135.0 * t7041;
    let t10293 = t3488 * t583;
    let t10294 = 4.0 / 45.0 * t10293;
    let t10295 = t1791 * t3553;
    let t10296 = t10295 * t661;
    let t10297 = t1621 * t10296;
    let t10299 = 4.0 / 15.0 * t639 * t10297;
    let t10300 = t1630 * t3499;
    let t10301 = t639 * t10300;
    let t10302 = 16.0 / 135.0 * t10301;
    let t10303 = 8.0 / 405.0 * t4993;
    let t10305 = 2.0 / 15.0 * t3479 * t663;
    (t10290, t10291, t10294, t10299, t10302, t10303, t10305)
}
