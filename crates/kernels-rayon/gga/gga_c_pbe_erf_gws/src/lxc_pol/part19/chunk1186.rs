//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1186/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1186(t15371: f64, t3989: f64, t1173: f64, t3909: f64, t3781: f64, t3950: f64, t850: f64, t833: f64, t3703: f64, t4066: f64, t1105: f64, t14852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15372 = t3989 * t15371;
    let t15374 = t1173 * t3909;
    let t15377 = t850 * t3781 * t3950;
    let t15378 = t15377 * t833;
    let t15397 = t4066 * t3703;
    let t15400 = t14852 * t1105;
    (t15372, t15374, t15377, t15378, t15397, t15400)
}
