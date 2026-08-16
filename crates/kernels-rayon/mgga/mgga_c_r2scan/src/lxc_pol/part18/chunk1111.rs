//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1111/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1111(t40177: f64, t261: f64, t3304: f64, t7309: f64, t10740: f64, t980: f64, t29418: f64, t3293: f64, t132: f64, t537: f64, t1575: f64, t25826: f64, t3342: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40178 = 0.69345773920434148506e0_f64 * t40177;
    let t40180 = t3304 * t261 * t7309;
    let t40181 = 0.69345773920434148506e0_f64 * t40180;
    let t40185 = t980 * t10740;
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40201 = t571 * t1575 * t3342 * t25826;
    (t40178, t40181, t40185, t40194, t40195, t40201)
}
