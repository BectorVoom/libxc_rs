//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 344/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk344(t322: f64, t1269: f64, t1271: f64, t1276: f64, t1277: f64, t1289: f64, t321: f64, t819: f64, t826: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t1291 = t1269 * t321 - 2.0_f64 * t1271 * t826 + 2.0_f64 * t1276 * t1277 - t819 * t1289;
    let t1292 = piecewise3(t324, 0.0_f64, t1291);
    (t1291, t1292)
}
