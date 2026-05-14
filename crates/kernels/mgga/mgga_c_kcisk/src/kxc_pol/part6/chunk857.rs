//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 857/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk857<F: Float>(t24073: F, t28961: F, t28966: F, t28970: F, t28973: F, t29326: F, t29330: F, t29334: F, t29338: F, t29340: F, t29343: F, t7648: F, t9235: F, t29770: F, t29782: F, t29958: F, t29971: F, t29988: F, t30003: F, t30020: F) -> (F,) {
    let t30034 = -0.79445938271604938269e-1 * t28961 - 0.10446625e-1 * t28966 - 0.27857666666666666666e-1 * t28970 + 0.34822083333333333333e-2 * t28973 + 0.46429444444444444443e-2 * t29326 + 0.51588271604938271604e-3 * t29330 - 0.69644166666666666666e-2 * t29334 + 0.58036805555555555556e-2 * t29338 - 0.579e0 * t7648 * t9235 - 0.69644166666666666665e-2 * t29340 - 0.52233124999999999998e-2 * t29343 - 0.18571777777777777778e-1 * t24073;
    let t30037 = t29770 + t29782 + t29958 + t29971 + t29988 + t30003 + t30020 + t30034;
    (t30037,)
}
