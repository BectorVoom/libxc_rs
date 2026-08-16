//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1295/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1295(t1695: f64, t212: f64, t60720: f64, t17974: f64, t2395: f64, t2376: f64, t339: f64, t5557: f64, t803: f64, t2391: f64, t17990: f64, t5570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61079 = t60720 * t212 * t1695;
    let t61081 = t17974 * t2395;
    let t61086 = t339 * t5557 * t2376;
    let t61087 = t61086 * t803;
    let t61089 = t17974 * t2391;
    let t61183 = t17990 * t5570;
    (t61079, t61081, t61086, t61087, t61089, t61183)
}
