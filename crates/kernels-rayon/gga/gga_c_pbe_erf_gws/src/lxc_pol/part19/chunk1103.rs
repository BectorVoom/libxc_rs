//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1103/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1103(t1176: f64, t2333: f64, t1180: f64, t371: f64, t915: f64, t3970: f64) -> (f64, f64, f64, f64) {
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13916 = t915 * t371;
    let t13917 = t3970 * t13916;
    (t13893, t13894, t13916, t13917)
}
