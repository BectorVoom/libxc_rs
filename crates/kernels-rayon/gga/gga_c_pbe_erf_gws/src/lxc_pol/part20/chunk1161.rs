//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1161/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1161(t2306: f64, t820: f64, t9385: f64, t3975: f64, t3972: f64, t13948: f64, t13954: f64, t13962: f64, t13964: f64, t14664: f64, t14669: f64, t14674: f64, t14678: f64, t14680: f64, t14685: f64, t14689: f64, t14693: f64, t3066: f64) -> (f64, f64, f64) {
    let t14696 = t2306 * t820;
    let t14697 = t9385 * t14696;
    let t14698 = t3975 * t14697;
    let t14699 = t3972 * t14698;
    let t14703 = t3066 * t14664 / 48.0_f64 + t3066 * t14669 / 48.0_f64 + t14674 / 96.0_f64 + t14678 / 96.0_f64 + t14680 / 96.0_f64 + t14685 / 1536.0_f64 - 7.0_f64 / 288.0_f64 * t14689 - t13948 - t14693 / 3072.0_f64 + 7.0_f64 / 288.0_f64 * t13954 + t14699 / 768.0_f64 + 7.0_f64 / 288.0_f64 * t13962 + 7.0_f64 / 4608.0_f64 * t13964;
    (t14696, t14698, t14703)
}
