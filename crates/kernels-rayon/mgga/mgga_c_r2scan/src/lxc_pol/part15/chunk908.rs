//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 908/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk908(t1010: f64, t1271: f64, t1276: f64, t1277: f64, t1289: f64, t2378: f64, t2381: f64, t2391: f64, t321: f64, t6651: f64, t6654: f64, t6661: f64, t819: f64, t826: f64, t8353: f64, t8355: f64, t8358: f64, t8367: f64, t8370: f64, t8373: f64, t8395: f64) -> f64 {
    let t8397 = -t6651 * t1010 - 2.0_f64 * t1271 * t2391 + 4.0_f64 * t1276 * t8370 + 2.0_f64 * t1276 * t8373 + 2.0_f64 * t8358 * t1277 - t2378 * t1289 + 4.0_f64 * t6654 * t2381 + t8353 * t321 - 6.0_f64 * t6661 * t8367 - t819 * t8395 - 2.0_f64 * t8355 * t826;
    t8397
}
