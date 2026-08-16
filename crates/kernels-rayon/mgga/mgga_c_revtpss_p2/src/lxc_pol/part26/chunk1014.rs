//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1014/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1014(t1246: f64, t13068: f64, t1250: f64, t12732: f64, t482: f64, t1042: f64, t1263: f64, t3568: f64, t1122: f64, t247: f64, t3372: f64, t3634: f64) -> (f64, f64, f64, f64) {
    let t13069 = t13068 * t1246;
    let t13075 = t482 * t12732 * t1250;
    let t13076 = t1042 * t13075;
    let t13079 = t1263 * t3568;
    let t13080 = t13079 * t1122;
    let t13081 = t1042 * t13080;
    let t13085 = t247 * t3634 * t3372;
    (t13069, t13076, t13081, t13085)
}
