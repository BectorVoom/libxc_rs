//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 751/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk751(t1770: f64, t395: f64, t1660: f64, t56: f64, t1662: f64, t259: f64, t43: f64, t1783: f64, t636: f64, t1841: f64, t735: f64, t155: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4947 = t395 * t1770;
    let t4949 = t56 * t1660;
    let t4951 = 1.0_f64 / t1662 / t259;
    let t4957 = 1.0_f64 / t1662 / t43;
    let t4985 = t1783 * t636;
    let t4987 = t1841 * t735;
    let t4991 = t155 * t589;
    (t4947, t4949, t4951, t4957, t4985, t4987, t4991)
}
