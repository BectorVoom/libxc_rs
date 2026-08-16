//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 640/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk640(t1048: f64, t3618: f64, t499: f64, t2867: f64, t3263: f64, t3275: f64, t1010: f64, t3358: f64, t1070: f64, t2378: f64, t1276: f64, t3357: f64, t3368: f64) -> (f64, f64, f64, f64) {
    let t3620 = t1048 * t499 * t3618;
    let t3621 = t3620 / 4.0_f64;
    let t3623 = t3275 * t3263 * t2867;
    let t3624 = t3623 / 4.0_f64;
    let t3625 = t3358 * t1010;
    let t3627 = t2378 * t1070;
    let t3629 = t1070 * t1010;
    let t3630 = t1276 * t3629;
    let t3632 = t3357 + t3625 / 8.0_f64 - t3627 / 8.0_f64 + t3630 / 4.0_f64 + t3368;
    (t3621, t3624, t3629, t3632)
}
