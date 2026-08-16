//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1298/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1298(t11509: f64, t3950: f64, t833: f64, t850: f64, t3959: f64, t9932: f64, t3897: f64, t4386: f64, t13792: f64, t15167: f64, t3972: f64, t50956: f64, t8827: f64) -> (f64, f64, f64, f64) {
    let t56773 = t850 * t11509 * t3950 * t833;
    let t56776 = t3959 * t9932;
    let t56782 = t4386 * t3897;
    let t56783 = t13792 * t56782;
    let t56787 = t3972 * t50956 * t8827 * t15167;
    (t56773, t56776, t56783, t56787)
}
