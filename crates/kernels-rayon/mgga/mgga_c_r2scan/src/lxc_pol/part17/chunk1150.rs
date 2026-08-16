//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1150/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1150(t11036: f64, t9657: f64, t1070: f64, t31764: f64, t2928: f64, t37028: f64, t11033: f64, t2938: f64, t3366: f64, t9640: f64, t2441: f64, t3675: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42534 = t11036 * t9657;
    let t42536 = t31764 * t1070;
    let t42539 = t37028 * t2928;
    let t42541 = t11033 * t2938;
    let t42543 = t9640 * t3366;
    let t42753 = t3675 * t2441;
    (t42534, t42536, t42539, t42541, t42543, t42753)
}
