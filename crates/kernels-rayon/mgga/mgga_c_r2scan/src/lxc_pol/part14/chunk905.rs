//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 905/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk905(t2376: f64, t818: f64, t1004: f64, t1275: f64, t1010: f64, t1277: f64, t2391: f64, t826: f64, t1289: f64, t1248: f64, t35: f64, t1256: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8355 = t2376 * t818;
    let t8358 = t1004 * t1275;
    let t8367 = t1010 * t1277;
    let t8370 = t2391 * t826;
    let t8373 = t1010 * t1289;
    let t8377 = t1248 * t35;
    let t8385 = t1256 * t35;
    (t8355, t8358, t8367, t8370, t8373, t8377, t8385)
}
