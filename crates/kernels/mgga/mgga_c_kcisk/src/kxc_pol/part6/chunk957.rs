//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 957/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk957<F: Float>(t29807: F, t29842: F, t29887: F, t29943: F, t2029: F, t1994: F, t28271: F, t28277: F, t28282: F, t28285: F, t28288: F, t28292: F, t28297: F, t28301: F, t28306: F, t28309: F, t28317: F) -> F {
    let t29945 = t29807 + t29842 + t29887 + t29943;
    let t29946 = t29945 * t2029;
    let t29958 = -F::cast_from(0.34822083333333333333e-2_f64) * t28271 - F::cast_from(0.34822083333333333333e-2_f64) * t28277 - F::cast_from(0.193e0_f64) * t1994 * t29946 + F::cast_from(0.23214722222222222222e-2_f64) * t28282 - F::cast_from(0.69644166666666666665e-2_f64) * t28285 - F::cast_from(0.18571777777777777778e-1_f64) * t28288 + F::cast_from(0.18571777777777777778e-1_f64) * t28292 - F::cast_from(0.34822083333333333333e-2_f64) * t28297 + F::cast_from(0.34048259259259259259e-1_f64) * t28301 + F::cast_from(0.11607361111111111111e-2_f64) * t28306 + F::cast_from(0.34822083333333333333e-2_f64) * t28309 + F::cast_from(0.11607361111111111111e-2_f64) * t28317;
    t29958
}
