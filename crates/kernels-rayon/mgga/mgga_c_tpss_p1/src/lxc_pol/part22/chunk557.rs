//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 557/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk557(t2406: f64, t2407: f64, t2157: f64, t246: f64, t768: f64, t806: f64, t2163: f64, t220: f64, t229: f64, t2365: f64, t2370: f64, t2398: f64, t339: f64, t783: f64, t813: f64) -> (f64, f64, f64) {
    let t2408 = t2406 * t2407;
    let t2411 = t2157 * t246;
    let t2415 = t768 * t806;
    let t2425 = 2.0_f64 * t2163 * t2411 * t339 + t220 * t229 * t2398 - t2365 * t339 * t813 - t2370 * t339 * t813 - 2.0_f64 * t2415 * t339 * t783;
    (t2408, t2415, t2425)
}
