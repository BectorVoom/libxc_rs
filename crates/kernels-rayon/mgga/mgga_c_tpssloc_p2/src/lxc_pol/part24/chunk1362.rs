//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1362/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1362(t3173: f64, t3175: f64, t1921: f64, t1054: f64, t3206: f64, t1920: f64, t23353: f64, t968: f64, t1049: f64, t23592: f64, t10164: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82441 = t3173 * t3175;
    let t82442 = t1921 * t82441;
    let t82457 = t1054 * t3206;
    let t82463 = t1920 * t968 * t23353;
    let t82469 = t23592 * t1049;
    let t82481 = t225 * t10164;
    (t82441, t82442, t82457, t82463, t82469, t82481)
}
