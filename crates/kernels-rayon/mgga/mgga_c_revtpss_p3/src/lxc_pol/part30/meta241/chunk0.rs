//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1079/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1079(t1010: f64, t1480: f64, t1715: f64, t3634: f64, t247: f64, t1261: f64, t1260: f64, t1785: f64, t3670: f64, t3719: f64, t5230: f64, t1802: f64, t369: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5373 = t1480 * t1010;
    let t5377 = t3634 * t1715;
    let t5378 = t247 * t5377;
    let t5379 = t1261 * t5378;
    let t5381 = t1785 * t1260;
    let t5384 = t3670 * t1260;
    let t5385 = t3719 * t5230;
    let t5386 = t247 * t5385;
    let t5389 = t1802 * t369;
    (t5373, t5378, t5379, t5381, t5384, t5386, t5389)
}
