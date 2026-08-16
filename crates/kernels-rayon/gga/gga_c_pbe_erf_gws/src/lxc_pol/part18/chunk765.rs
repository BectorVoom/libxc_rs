//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 765/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk765(t4939: f64, t1243: f64, t574: f64, t1660: f64, t56: f64, t1662: f64, t259: f64, t43: f64, t155: f64, t589: f64, t592: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4940 = 0.58774074074074074074e-2_f64 * t4939;
    let t4941 = t1243 * t574;
    let t4949 = t56 * t1660;
    let t4951 = 1.0_f64 / t1662 / t259;
    let t4957 = 1.0_f64 / t1662 / t43;
    let t4991 = t155 * t589;
    let t4992 = t4991 * t592;
    let t4993 = t587 * t4992;
    (t4940, t4941, t4949, t4951, t4957, t4991, t4993)
}
