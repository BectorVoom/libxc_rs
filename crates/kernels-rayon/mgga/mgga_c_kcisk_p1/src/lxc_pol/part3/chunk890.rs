//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 890/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk890(t1163: f64, t4174: f64, t3484: f64, t3482: f64, t1440: f64, t3502: f64, t1450: f64, t1415: f64, t1411: f64, t3739: f64, t3779: f64, t1412: f64) -> (f64, f64, f64, f64, f64) {
    let t13316 = t4174 * t1163;
    let t13317 = t3484 * t13316;
    let t13318 = t3482 * t13317;
    let t13320 = t3502 * t1440;
    let t13321 = t1450 * t13320;
    let t13322 = t1415 * t13321;
    let t13323 = t1411 * t13322;
    let t13325 = t3739 * t3779;
    let t13327 = t1412 * t1412;
    (t13318, t13320, t13323, t13325, t13327)
}
