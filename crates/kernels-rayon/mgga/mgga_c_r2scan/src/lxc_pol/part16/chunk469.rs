//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 469/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk469(t322: f64, t1010: f64, t1271: f64, t1276: f64, t2376: f64, t2378: f64, t2381: f64, t2391: f64, t321: f64, t819: f64, t826: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t2393 = -t1271 * t1010 + 2.0_f64 * t1276 * t2381 + t2376 * t321 - t2378 * t826 - t819 * t2391;
    let t2394 = piecewise3(t324, 0.0_f64, t2393);
    (t2393, t2394)
}
