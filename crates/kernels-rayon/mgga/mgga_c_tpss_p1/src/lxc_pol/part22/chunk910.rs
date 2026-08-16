//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 910/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk910(t8274: f64, t230: f64, t2162: f64, t226: f64, t2376: f64, t339: f64, t769: f64, t785: f64, t2169: f64, t2372: f64, t2158: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8275 = 1.0_f64 / t8274;
    let t8276 = t8275 * t230;
    let t8279 = t2162 * t226;
    let t8286 = t339 * t769 * t2376;
    let t8287 = t8286 * t785;
    let t8289 = t2169 * t2372;
    let t8292 = t339 * t2158 * t789;
    (t8275, t8276, t8279, t8286, t8287, t8289, t8292)
}
