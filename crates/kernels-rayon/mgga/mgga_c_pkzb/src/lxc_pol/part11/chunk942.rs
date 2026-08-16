//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 942/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk942(t3757: f64, t6404: f64, t824: f64, t758: f64, t3026: f64, t3236: f64, t179: f64, t6398: f64, t404: f64, t2405: f64, t3730: f64, t1238: f64, t3229: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10261 = t6404 * t3757;
    let t10262 = t10261 * t824;
    let t10263 = t758 * t10262;
    let t10266 = t3236 * t3026;
    let t10267 = t758 * t10266;
    let t10271 = t179 * t6398 * t3757;
    let t10272 = t404 * t10271;
    let t10275 = t179 * t2405 * t3730;
    let t10276 = t404 * t10275;
    let t10278 = t1238 * t3229;
    (t10261, t10262, t10263, t10266, t10267, t10271, t10272, t10275, t10276, t10278)
}
