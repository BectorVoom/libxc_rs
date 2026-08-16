//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1008/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1008(t235: f64, t25160: f64, t4234: f64, t6657: f64, t25249: f64, t829: f64, t6646: f64, t22986: f64, t22996: f64, t4283: f64, t1888: f64, t1484: f64, t23153: f64) -> (f64, f64, f64, f64, f64) {
    let t25295 = t235 * t25160;
    let t25297 = t6657 * t4234;
    let t25299 = t25249 * t829;
    let t25300 = t6646 * t25299;
    let t25301 = t22986 * t25300;
    let t25303 = t22996 * t4283;
    let t25304 = t1888 * t25303;
    let t25306 = t23153 * t1484;
    (t25295, t25297, t25301, t25304, t25306)
}
