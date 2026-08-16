//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1109/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1109(t2453: f64, t555: f64, t25304: f64, t1444: f64, t543: f64, t1419: f64, t7063: f64, t25081: f64, t7234: f64, t198: f64, t206: f64, t7427: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94382 = t2453 * t555;
    let t94390 = t25304 * t555;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    let t95088 = t7234 * t25081;
    let t95511 = t198 * t206 * t7427;
    (t94382, t94390, t94396, t94801, t95088, t95511)
}
