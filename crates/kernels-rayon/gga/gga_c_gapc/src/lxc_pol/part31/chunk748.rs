//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 748/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk748(t8566: f64, t8567: f64, t2952: f64, t4015: f64, t4018: f64, t8362: f64, t1595: f64, t2890: f64, t473: f64, t1037: f64, t1416: f64, t4687: f64) -> (f64, f64, f64, f64, f64) {
    let t8568 = t8566 * t8567;
    let t8570 = t2952 * t4015;
    let t8571 = t8362 * t4018;
    let t8572 = t8570 * t8571;
    let t8574 = t2890 * t1595;
    let t8575 = t473 * t8574;
    let t8577 = t1416 * t1037;
    let t8578 = t8577 * t4687;
    (t8568, t8570, t8572, t8575, t8578)
}
