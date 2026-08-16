//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 957/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk957(t29807: f64, t29842: f64, t29887: f64, t29943: f64, t2029: f64, t1994: f64, t28271: f64, t28277: f64, t28282: f64, t28285: f64, t28288: f64, t28292: f64, t28297: f64, t28301: f64, t28306: f64, t28309: f64, t28317: f64) -> f64 {
    let t29945 = t29807 + t29842 + t29887 + t29943;
    let t29946 = t29945 * t2029;
    let t29958 = -0.34822083333333333333e-2_f64 * t28271 - 0.34822083333333333333e-2_f64 * t28277 - 0.193e0_f64 * t1994 * t29946 + 0.23214722222222222222e-2_f64 * t28282 - 0.69644166666666666665e-2_f64 * t28285 - 0.18571777777777777778e-1_f64 * t28288 + 0.18571777777777777778e-1_f64 * t28292 - 0.34822083333333333333e-2_f64 * t28297 + 0.34048259259259259259e-1_f64 * t28301 + 0.11607361111111111111e-2_f64 * t28306 + 0.34822083333333333333e-2_f64 * t28309 + 0.11607361111111111111e-2_f64 * t28317;
    t29958
}
