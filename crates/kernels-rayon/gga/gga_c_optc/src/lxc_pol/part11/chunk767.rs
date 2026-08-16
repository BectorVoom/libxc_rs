//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 767/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk767(t1464: f64, t2934: f64, t1446: f64, t3017: f64, t1519: f64, t7878: f64, t1133: f64, t1523: f64, t3169: f64, t116: f64, t3241: f64, t3242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12268 = t1464 * t2934;
    let t12366 = t1446 * t3017;
    let t12489 = t7878 * t1519;
    let t12490 = t1133 * t12489;
    let t12522 = t1523 * t3169;
    let t12567 = t3241 * t3242 * t116;
    (t12268, t12366, t12489, t12490, t12522, t12567)
}
