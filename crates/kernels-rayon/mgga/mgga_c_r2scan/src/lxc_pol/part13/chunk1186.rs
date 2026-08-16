//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1186/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1186(t37982: f64, t7620: f64, t10856: f64, t7407: f64, t38153: f64, t10868: f64, t2147: f64, t8066: f64, t7470: f64, t38144: f64, t38147: f64, t38150: f64, t38156: f64, t38158: f64, t38161: f64, t38165: f64) -> f64 {
    let t40232 = t37982 * t7620;
    let t40233 = 0.19514881078765566037e-1_f64 * t40232;
    let t40234 = t10856 * t7407;
    let t40238 = 0.57829097596741960692e-3_f64 * t38153;
    let t40241 = t2147 * t10868 * t8066;
    let t40242 = 0.46574606203128791246e-1_f64 * t40241;
    let t40243 = t10856 * t7470;
    let t40244 = 0.19514881078765566037e-1_f64 * t40243;
    let t40247 = t40233 + 0.58544643236296698113e-1_f64 * t40234 - t38144 + 0.46574606203128791246e-1_f64 * t38147 + 0.16262400898971305032e-3_f64 * t38150 - t40238 + 0.13972381860938637374e0_f64 * t38156 - t40242 - t40244 + 0.64025200389650807209e-1_f64 * t38158 - 0.46574606203128791246e-1_f64 * t38161 + t38165;
    t40247
}
