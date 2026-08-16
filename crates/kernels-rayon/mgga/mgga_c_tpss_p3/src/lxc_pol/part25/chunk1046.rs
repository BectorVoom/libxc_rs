//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1046/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1046(t13335: f64, t836: f64, t861: f64, t141: f64, t4573: f64, t8444: f64, t581: f64, t2457: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t14452 = t836 * t13335;
    let t14453 = t861 * t14452;
    let t14454 = t141 * t14453;
    let t14456 = t8444 * t4573;
    let t14457 = t14456 * t581;
    let t14458 = t2457 * t14457;
    let t14459 = t128 * t14458;
    (t14452, t14454, t14457, t14459)
}
