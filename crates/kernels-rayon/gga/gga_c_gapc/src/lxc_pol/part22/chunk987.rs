//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 987/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk987(t11442: f64, t5553: f64, t11326: f64, t3714: f64, t116: f64, t190: f64, t1: f64, t102: f64, t3694: f64) -> (f64, f64, f64, f64) {
    let t11443 = t5553 * t11442;
    let t11445 = t11326 * t3714;
    let t11447 = t116 * t190;
    let t11449 = t3694 * t1 * t102;
    (t11443, t11445, t11447, t11449)
}
