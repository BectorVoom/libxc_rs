//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1199/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1199(t1765: f64, t3670: f64, t11391: f64, t3163: f64, t128: f64, t203: f64, t11417: f64, t457: f64, t5741: f64, t632: f64, t1781: f64, t3684: f64) -> (f64, f64, f64, f64, f64) {
    let t34858 = t3670 * t1765;
    let t34860 = t11391 * t3163;
    let t34863 = t203 * t128;
    let t34866 = t632 * t11417 * t5741 * t34863 * t457;
    let t34868 = t3684 * t1781;
    (t34858, t34860, t34863, t34866, t34868)
}
