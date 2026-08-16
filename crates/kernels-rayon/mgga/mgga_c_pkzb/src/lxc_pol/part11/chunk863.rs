//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 863/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk863(t3515: f64, t759: f64, t761: f64, t2105: f64, t2106: f64, t3679: f64, t287: f64, t2916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9268 = t3515 * t759;
    let t9269 = t9268 * t761;
    let t9270 = t2105 * t9269;
    let t9273 = t3679 * t2106;
    let t9274 = t2105 * t9273;
    let t9277 = t287 * t2916;
    (t9268, t9269, t9270, t9273, t9274, t9277)
}
