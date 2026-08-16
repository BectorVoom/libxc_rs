//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1180/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1180(t261: f64, t3299: f64, t7390: f64, t10879: f64, t11727: f64, t3304: f64, t7309: f64, t10760: f64, t2147: f64, t24059: f64, t10740: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t40175 = t3299 * t261 * t7390;
    let t40176 = 0.23115257973478049502e0_f64 * t40175;
    let t40177 = t10879 * t11727;
    let t40178 = 0.69345773920434148506e0_f64 * t40177;
    let t40180 = t3304 * t261 * t7309;
    let t40181 = 0.69345773920434148506e0_f64 * t40180;
    let t40183 = t2147 * t10760 * t24059;
    let t40185 = t980 * t10740;
    (t40176, t40178, t40181, t40183, t40185)
}
