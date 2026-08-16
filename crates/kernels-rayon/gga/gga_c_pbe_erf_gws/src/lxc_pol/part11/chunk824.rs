//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 824/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk824(t10025: f64, t12335: f64, t12336: f64, t12337: f64, t12363: f64, t12364: f64, t12365: f64, t4652: f64, t4664: f64, t4744: f64, t4751: f64, t4754: f64, t4784: f64, t6076: f64) -> f64 {
    let t13150 = 0.1232289865202e1_f64 * t10025;
    let t13151 = t12335 + t12336 - t12337 + t4744 + t4751 + t4652 + t4754 + t12363 + t4664 - t6076 + t12364 - t12365 - t13150 - t4784;
    t13151
}
