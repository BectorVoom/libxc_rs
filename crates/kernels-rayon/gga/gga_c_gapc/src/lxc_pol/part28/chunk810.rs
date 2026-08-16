//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 810/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk810(t9254: f64, t9256: f64, t1030: f64, t9253: f64, t1648: f64, t681: f64, t1038: f64, t5216: f64) -> (f64, f64, f64, f64, f64) {
    let t9257 = t9254 * t9256;
    let t9259 = t1030 * t9253;
    let t9260 = t1648 * t681;
    let t9261 = t1038 * t5216;
    let t9262 = t9260 * t9261;
    (t9257, t9259, t9260, t9261, t9262)
}
