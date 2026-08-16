//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1216/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1216(t107377: f64, t107381: f64, t107385: f64, t107389: f64, t107397: f64, t107402: f64, t107406: f64, t1336: f64, t19815: f64, t20568: f64, t29343: f64, t29345: f64, t5234: f64, t6378: f64, t7208: f64, t7932: f64, t7934: f64, t84595: f64, t84597: f64, t97148: f64, t97161: f64, t97179: f64, t97200: f64) -> f64 {
    let t107951 = 0.11514538467937585055e0_f64 * t97148 - 0.14804406601634037928e0_f64 * t97161 - 0.3289868133696452873e-1_f64 * t107377 - 3.0_f64 * t5234 * t29343 - 0.19739208802178717238e0_f64 * t107381 - t84595 - 0.29608813203268075857e0_f64 * t107385 + 0.9869604401089358619e-1_f64 * t107389 + t84597 - t1336 * t7208 * t20568 + 3.0_f64 * t6378 * t7934 - 3.0_f64 * t19815 * t7932 - 0.49348022005446793095e-1_f64 * t107397 - 0.69087230807625510332e0_f64 * t97179 + 0.16449340668482264365e-1_f64 * t107402 - 3.0_f64 * t5234 * t29345 - 0.16449340668482264365e-1_f64 * t107406 - 0.11514538467937585055e0_f64 * t97200;
    t107951
}
