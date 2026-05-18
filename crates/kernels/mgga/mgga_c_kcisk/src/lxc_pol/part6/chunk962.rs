//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 962/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk962<F: Float>(t24073: F, t28961: F, t28966: F, t28970: F, t28973: F, t29326: F, t29330: F, t29334: F, t29338: F, t29340: F, t29343: F, t7648: F, t9235: F) -> F {
    let t30034 = -F::new(0.79445938271604938269e-1) * t28961 - F::new(0.10446625e-1) * t28966 - F::new(0.27857666666666666666e-1) * t28970 + F::new(0.34822083333333333333e-2) * t28973 + F::new(0.46429444444444444443e-2) * t29326 + F::new(0.51588271604938271604e-3) * t29330 - F::new(0.69644166666666666666e-2) * t29334 + F::new(0.58036805555555555556e-2) * t29338 - F::new(0.579e0) * t7648 * t9235 - F::new(0.69644166666666666665e-2) * t29340 - F::new(0.52233124999999999998e-2) * t29343 - F::new(0.18571777777777777778e-1) * t24073;
    t30034
}
