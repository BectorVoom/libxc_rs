//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 660/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk660(t126: f64, t667: f64, t1463: f64, t457: f64, t1672: f64, t567: f64, t1180: f64, t5462: f64) -> (f64, f64, f64, f64) {
    let t5542 = t126 * t667;
    let t5544 = t1463 * t457;
    let t5549 = t1672 * t567;
    let t5553 = t5462 * t1180;
    (t5542, t5544, t5549, t5553)
}
