//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 640/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk640(t1060: f64, t3613: f64, t783: f64, t1010: f64, t3358: f64, t1070: f64, t2378: f64, t1276: f64, t1035: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3615 = t783 * t3613 * t1060;
    let t3625 = t3358 * t1010;
    let t3627 = t2378 * t1070;
    let t3629 = t1070 * t1010;
    let t3630 = t1276 * t3629;
    let t3675 = t352 * t1035;
    (t3615, t3625, t3627, t3629, t3630, t3675)
}
