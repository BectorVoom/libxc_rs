//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 987/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk987(t32063: f64, t32908: f64, t7366: f64, t1369: f64, t32929: f64, t376: f64, t32933: f64, t23649: f64, t32948: f64, t2: f64, t32869: f64, t32984: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t139361 = t7366 * t32063 * t32908;
    let t139377 = t1369 * t376 * t32929;
    let t139380 = t1369 * t376 * t32933;
    let t139390 = t23649 * t32948;
    let t139392 = t2 * t32869;
    let t139410 = t89 * t376 * t32984;
    (t139361, t139377, t139380, t139390, t139392, t139410)
}
