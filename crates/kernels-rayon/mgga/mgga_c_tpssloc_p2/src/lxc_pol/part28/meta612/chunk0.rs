//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1925/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1925(t22863: f64, t7737: f64, t26448: f64, t90497: f64, t215: f64, t6916: f64, t225: f64, t3787: f64, t562: f64, t16313: f64, t22751: f64, t26385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91000 = t22863 * t7737;
    let t91002 = t90497 * t26448;
    let t91004 = t6916 * t215;
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91008 = t91004 * t91006 * t16313;
    let t91010 = t22751 * t26385;
    (t91000, t91002, t91004, t91005, t91008, t91010)
}
