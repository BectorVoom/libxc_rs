//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 547/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk547(t127: f64, t1789: f64, t371: f64, t1235: f64, t1219: f64, t1778: f64, t1010: f64, t1480: f64, t1715: f64, t3634: f64, t247: f64, t1261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5362 = t371 * t127 * t1789;
    let t5363 = t1235 * t5362;
    let t5366 = t1778 * t1219;
    let t5373 = t1480 * t1010;
    let t5377 = t3634 * t1715;
    let t5378 = t247 * t5377;
    let t5379 = t1261 * t5378;
    (t5362, t5363, t5366, t5373, t5378, t5379)
}
