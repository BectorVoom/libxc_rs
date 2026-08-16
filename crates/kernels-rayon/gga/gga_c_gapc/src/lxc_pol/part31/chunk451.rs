//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 451/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk451(t126: f64, t2435: f64, t824: f64, t190: f64, t291: f64, t329: f64, t442: f64, t891: f64) -> (f64, f64, f64, f64, f64) {
    let t2436 = t2435 * t126;
    let t2437 = t824 * t2436;
    let t2438 = t190 * t291;
    let t2439 = t2438 * t329;
    let t2440 = t891 * t442;
    (t2436, t2437, t2438, t2439, t2440)
}
