//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 872/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk872(t1662: f64, t1763: f64, t16669: f64, t11: f64, t4949: f64, t1403: f64, t1407: f64, t4951: f64, t1243: f64, t1766: f64, t395: f64, t4959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16712 = 1.0_f64 / t1662 / t1763;
    let t16713 = t16712 * t16669;
    let t16715 = t11 * t4949 * t16713;
    let t16718 = t4951 * t1403 * t1407;
    let t16720 = t11 * t4949 * t16718;
    let t16722 = t1243 * t1766;
    let t16724 = t395 * t4959;
    (t16712, t16713, t16715, t16718, t16720, t16722, t16724)
}
