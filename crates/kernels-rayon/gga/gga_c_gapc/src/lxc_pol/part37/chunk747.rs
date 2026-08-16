//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 747/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk747(t2936: f64, t8578: f64, t2948: f64, t518: f64, t1460: f64, t2954: f64, t1404: f64, t2880: f64, t120: f64, t118: f64, t1803: f64, t61: f64) -> (f64, f64, f64, f64, f64) {
    let t8579 = t2936 * t8578;
    let t8581 = t518 * t2948;
    let t8583 = t1460 * t2954;
    let t8585 = t2880 * t1404;
    let t8586 = t120 * t8585;
    let t8588 = t1803 * t118;
    let t8589 = t61 * t8588;
    (t8579, t8581, t8583, t8586, t8589)
}
