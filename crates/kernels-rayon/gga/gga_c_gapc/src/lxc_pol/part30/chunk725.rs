//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 725/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk725(t8570: f64, t8571: f64, t1595: f64, t2890: f64, t473: f64, t1037: f64, t1416: f64, t4687: f64, t2936: f64, t2948: f64, t518: f64, t1460: f64, t2954: f64) -> (f64, f64, f64, f64, f64) {
    let t8572 = t8570 * t8571;
    let t8574 = t2890 * t1595;
    let t8575 = t473 * t8574;
    let t8577 = t1416 * t1037;
    let t8578 = t8577 * t4687;
    let t8579 = t2936 * t8578;
    let t8581 = t518 * t2948;
    let t8583 = t1460 * t2954;
    (t8572, t8575, t8579, t8581, t8583)
}
