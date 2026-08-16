//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 889/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk889(t785: f64, t8286: f64, t2158: f64, t339: f64, t789: f64, t2387: f64, t72: f64, t240: f64, t769: f64, t790: f64, t2162: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8287 = t8286 * t785;
    let t8292 = t339 * t2158 * t789;
    let t8305 = t2387 * t72;
    let t8306 = t8305 * t240;
    let t8313 = t339 * t769 * t790;
    let t8325 = t2162 * t750;
    (t8287, t8292, t8305, t8306, t8313, t8325)
}
