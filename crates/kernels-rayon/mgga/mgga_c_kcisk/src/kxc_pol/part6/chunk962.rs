//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 962/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk962(t24073: f64, t28961: f64, t28966: f64, t28970: f64, t28973: f64, t29326: f64, t29330: f64, t29334: f64, t29338: f64, t29340: f64, t29343: f64, t7648: f64, t9235: f64) -> f64 {
    let t30034 = -0.79445938271604938269e-1_f64 * t28961 - 0.10446625e-1_f64 * t28966 - 0.27857666666666666666e-1_f64 * t28970 + 0.34822083333333333333e-2_f64 * t28973 + 0.46429444444444444443e-2_f64 * t29326 + 0.51588271604938271604e-3_f64 * t29330 - 0.69644166666666666666e-2_f64 * t29334 + 0.58036805555555555556e-2_f64 * t29338 - 0.579e0_f64 * t7648 * t9235 - 0.69644166666666666665e-2_f64 * t29340 - 0.52233124999999999998e-2_f64 * t29343 - 0.18571777777777777778e-1_f64 * t24073;
    t30034
}
