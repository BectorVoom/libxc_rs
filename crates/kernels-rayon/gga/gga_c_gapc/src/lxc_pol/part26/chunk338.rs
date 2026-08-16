//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 338/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk338(t1462: f64, t1464: f64, t101: f64, t492: f64, t472: f64, t643: f64, t8: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t1465 = t1462 * t1464;
    let t1468 = t492 * t101;
    let t1469 = t1468 * t472;
    let t1473 = 1.0_f64 / t8 / t643;
    let t1474 = t5 * t1473;
    (t1465, t1468, t1469, t1473, t1474)
}
