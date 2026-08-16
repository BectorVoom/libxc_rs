//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1052/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1052(t36985: f64, t97: f64, t1275: f64, t502: f64, t263: f64, t6660: f64, t321: f64, t6100: f64, t1266: f64, t818: f64, t826: f64, t11056: f64, t1271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36986 = t97 * t36985;
    let t37028 = t502 * t1275;
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    let t37066 = t1271 * t11056;
    (t36986, t37028, t37031, t37038, t37040, t37041, t37066)
}
