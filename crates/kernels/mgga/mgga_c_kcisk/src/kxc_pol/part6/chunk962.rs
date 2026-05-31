//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 962/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk962<F: Float>(t24073: F, t28961: F, t28966: F, t28970: F, t28973: F, t29326: F, t29330: F, t29334: F, t29338: F, t29340: F, t29343: F, t7648: F, t9235: F) -> F {
    let t30034 = -F::cast_from(0.79445938271604938269e-1_f64) * t28961 - F::cast_from(0.10446625e-1_f64) * t28966 - F::cast_from(0.27857666666666666666e-1_f64) * t28970 + F::cast_from(0.34822083333333333333e-2_f64) * t28973 + F::cast_from(0.46429444444444444443e-2_f64) * t29326 + F::cast_from(0.51588271604938271604e-3_f64) * t29330 - F::cast_from(0.69644166666666666666e-2_f64) * t29334 + F::cast_from(0.58036805555555555556e-2_f64) * t29338 - F::cast_from(0.579e0_f64) * t7648 * t9235 - F::cast_from(0.69644166666666666665e-2_f64) * t29340 - F::cast_from(0.52233124999999999998e-2_f64) * t29343 - F::cast_from(0.18571777777777777778e-1_f64) * t24073;
    t30034
}
