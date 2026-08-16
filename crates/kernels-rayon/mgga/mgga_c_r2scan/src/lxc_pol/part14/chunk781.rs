//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 781/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk781(t1343: f64, t352: f64, t1347: f64, t349: f64, t854: f64, t2065: f64, t2271: f64, t2321: f64, t607: f64, t1783: f64, t879: f64, t2288: f64, t6007: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6751 = t352 * t1343;
    let t6755 = 1.0_f64 / t1347 / t349;
    let t6767 = 1.0_f64 / t1347 / t854;
    let t6794 = t2271 * t2065;
    let t6798 = t2321 * t607;
    let t6801 = t879 * t1783;
    let t6804 = t2288 * t6007;
    (t6751, t6755, t6767, t6794, t6798, t6801, t6804)
}
