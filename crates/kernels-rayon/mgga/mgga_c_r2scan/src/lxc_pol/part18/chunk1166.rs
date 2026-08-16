//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1166/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1166(t31393: f64, t795: f64, t3263: f64, t3275: f64, t113: f64, t40393: f64, t97: f64, t11510: f64, t11487: f64, t40282: f64, t3579: f64, t40473: f64) -> (f64, f64, f64, f64) {
    let t42940 = t31393 * t795;
    let t42943 = t3275 * t3263 * t42940 / 2.0_f64;
    let t42945 = t97 * t40393 * t113;
    let t42947 = 3.0_f64 * t42945 * t11510;
    let t42949 = 15.0_f64 / 8.0_f64 * t40282 * t11487;
    let t42951 = 5.0_f64 / 8.0_f64 * t3579 * t40473;
    (t42943, t42947, t42949, t42951)
}
