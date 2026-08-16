//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1323/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1323(t6210: f64, t959: f64, t11687: f64, t6951: f64, t11682: f64, t6943: f64, t11683: f64, t23579: f64, t11632: f64, t2245: f64, t6201: f64, t11633: f64, t2208: f64, t24181: f64) -> (f64, f64, f64, f64, f64) {
    let t35790 = t6210 * t959;
    let t35792 = t11687 * t35790 * t6951;
    let t35795 = t11682 * t35790 * t6943;
    let t35798 = t11682 * t11683 * t23579;
    let t35801 = t11632 * t2245 * t6201;
    let t35806 = t24181 * t2208 * t11633;
    (t35792, t35795, t35798, t35801, t35806)
}
