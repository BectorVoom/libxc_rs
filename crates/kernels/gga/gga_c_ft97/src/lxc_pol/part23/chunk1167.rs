//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1167/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1167<F: Float>(t29073: F, t8392: F, t852: F, t9568: F, t25162: F, t28761: F, t10570: F, t192: F, t10491: F, t1476: F, t43917: F, t43912: F, t28782: F, t28719: F, t668: F, t1486: F, t28502: F, t681: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113017 = 4.0 / 3.0 * t8392 * t29073;
    let t113055 = t9568 * t852;
    let t113060 = t25162 * t28761;
    let t113061 = 2.0 / 9.0 * t113060;
    let t113070 = t192 * t10570;
    let t113076 = t10491 * t1476;
    let t113080 = t43917 * t1476;
    let t113101 = t43912 * t1476;
    let t113105 = t25162 * t28782;
    let t113106 = 2.0 * t113105;
    let t113141 = t28719 * t668;
    let t113168 = t1486 * t681 * t28502;
    (t113017, t113055, t113060, t113061, t113070, t113076, t113080, t113101, t113105, t113106, t113141, t113168)
}
