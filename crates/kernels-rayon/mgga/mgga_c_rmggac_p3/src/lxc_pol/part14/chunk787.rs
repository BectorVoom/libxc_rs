//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 787/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk787(t2185: f64, t7407: f64, t7411: f64, t4028: f64, t668: f64, t507: f64, t8629: f64, t124: f64, t338: f64, t22: f64, t235: f64, t504: f64, t7191: f64) -> (f64, f64, f64, f64, f64) {
    let t36612 = t7407 * t2185;
    let t36613 = t36612 * t7411;
    let t36624 = t4028 * t668;
    let t36629 = t507 * t8629;
    let t36632 = t124 * t338;
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    (t36613, t36624, t36629, t36634, t36639)
}
