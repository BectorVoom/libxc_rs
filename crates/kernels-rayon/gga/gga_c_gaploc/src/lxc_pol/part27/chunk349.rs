//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 349/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk349(t1508: f64, t189: f64, t200: f64, t23: f64, t203: f64, t61: f64, t172: f64, t911: f64, t107: f64, t1328: f64, t600: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1509 = t1508 * t189;
    let t1512 = t23 * t200;
    let t1514 = t61 * t1512 * t203;
    let t1519 = t911 * t172;
    let t1520 = t107 * t1519;
    let t1525 = t600 * t1328;
    let t1526 = t568 * t1525;
    (t1509, t1512, t1514, t1519, t1520, t1526)
}
