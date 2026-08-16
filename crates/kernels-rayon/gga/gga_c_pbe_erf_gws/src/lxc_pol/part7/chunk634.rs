//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 634/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk634(t1660: f64, t56: f64, t1662: f64, t259: f64, t4352: f64, t11: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4949 = t56 * t1660;
    let t4951 = 1.0_f64 / t1662 / t259;
    let t4952 = t4951 * t4352;
    let t4953 = t4949 * t4952;
    let t4954 = t11 * t4953;
    let t4957 = 1.0_f64 / t1662 / t43;
    (t4949, t4951, t4952, t4953, t4954, t4957)
}
