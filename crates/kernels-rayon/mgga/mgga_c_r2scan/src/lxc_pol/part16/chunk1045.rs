//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1045/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1045(t36985: f64, t97: f64, t1299: f64, t3370: f64, t1074: f64, t6692: f64, t1275: f64, t502: f64, t263: f64, t6660: f64, t321: f64, t6100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36986 = t97 * t36985;
    let t37020 = t3370 * t1299;
    let t37023 = t1074 * t6692;
    let t37028 = t502 * t1275;
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    (t36986, t37020, t37023, t37028, t37031, t37038)
}
