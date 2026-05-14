//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 881/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk881<F: Float>(t2801: F, t6556: F, t2355: F, t2902: F, t3366: F, t4342: F, t605: F, t4349: F, t921: F, t1382: F, t1016: F, t2497: F, t1377: F, t3418: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10290 = t6556 * t2801;
    let t10291 = 2.0 * t10290;
    let t10292 = t2355 * t2902;
    let t10293 = t4342 * t3366;
    let t10294 = 2.0 * t10293;
    let t10295 = t3366 * t605;
    let t10296 = t4349 * t10295;
    let t10297 = 6.0 * t10296;
    let t10298 = t2902 * t921;
    let t10299 = t1382 * t10298;
    let t10300 = 2.0 * t10299;
    let t10301 = t1016 * t2497;
    let t10302 = t1382 * t10301;
    let t10303 = 2.0 * t10302;
    let t10304 = t1377 * t3418;
    let t10305 = t3418 * t605;
    (t10290, t10291, t10292, t10293, t10294, t10295, t10296, t10297, t10298, t10299, t10300, t10301, t10302, t10303, t10304, t10305)
}
