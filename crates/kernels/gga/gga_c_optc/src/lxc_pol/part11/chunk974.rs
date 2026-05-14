//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 974/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk974<F: Float>(t3386: F, t6636: F, t1294: F, t23017: F, t1278: F, t6923: F, t1274: F, t22797: F, t1261: F, t22850: F, t1339: F, t3696: F) -> (F, F, F, F, F, F) {
    let t29367 = t3386 * t6636;
    let t29441 = t23017 * t1294;
    let t29592 = t6923 * t1278;
    let t29750 = t22797 * t1274;
    let t29752 = t22850 * t1261;
    let t30189 = t3696 * t1339;
    (t29367, t29441, t29592, t29750, t29752, t30189)
}
