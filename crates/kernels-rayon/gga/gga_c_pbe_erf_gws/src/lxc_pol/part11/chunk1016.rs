//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1016/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1016(t12571: f64, t1651: f64, t587: f64, t12778: f64, t17252: f64, t12612: f64, t1620: f64, t4934: f64, t12616: f64, t5137: f64, t639: f64, t10927: f64, t2612: f64) -> (f64, f64, f64, f64, f64) {
    let t41223 = t587 * t1651 * t12571;
    let t41245 = t587 * t17252 * t12778;
    let t41297 = t1620 * t4934 * t12612;
    let t41300 = t639 * t5137 * t12616;
    let t41326 = t2612 * t10927;
    (t41223, t41245, t41297, t41300, t41326)
}
