//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 193/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk193(t912: f64, t913: f64, t587: f64, t600: f64, t874: f64, t568: f64, t193: f64, t557: f64, t574: f64, t597: f64, t895: f64, t902: f64, t904: f64, t908: f64) -> (f64, f64, f64, f64, f64) {
    let t914 = t912 * t913;
    let t915 = t587 * t914;
    let t917 = t600 * t874;
    let t918 = t568 * t917;
    let t921 = 0.35750489951850426669e0_f64 * t895 * t193 + 0.14896037479937677779e-1_f64 * t902 - 0.35750489951850426669e0_f64 * t557 * t904 - 0.23005755572352449806e1_f64 * t574 * t908 - 0.95857314884801874192e-1_f64 * t915 + 0.23005755572352449806e1_f64 * t597 * t918;
    (t914, t915, t917, t918, t921)
}
