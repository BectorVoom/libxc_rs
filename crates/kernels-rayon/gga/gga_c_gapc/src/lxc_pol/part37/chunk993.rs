//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 993/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk993(t11463: f64, t11465: f64, t3060: f64, t3687: f64, t1040: f64, t3065: f64, t3688: f64, t3071: f64, t474: f64, t1030: f64, t3076: f64, t11326: f64, t3144: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11466 = t11463 * t11465;
    let t11468 = t3060 * t3687;
    let t11469 = t11468 * t1040;
    let t11471 = t3688 * t3065;
    let t11473 = t474 * t3071;
    let t11474 = t1030 * t11473;
    let t11475 = t11474 * t3076;
    let t11477 = t11326 * t3144;
    (t11466, t11468, t11469, t11471, t11473, t11474, t11475, t11477)
}
