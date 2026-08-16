//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 877/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk877(t254: f64, t6: f64, t6469: f64, t2323: f64, t3268: f64, t1113: f64, t904: f64) -> (f64, f64, f64) {
    let t9482 = t254 * t6 * t6469;
    let t9498 = 7.0_f64 / 576.0_f64 * t2323 * t3268;
    let t9499 = t904 * t1113;
    (t9482, t9498, t9499)
}
