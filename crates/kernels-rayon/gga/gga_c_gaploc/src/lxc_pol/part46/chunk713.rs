//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 713/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk713(t13096: f64, t314: f64, t313: f64, t739: f64, t531: f64, t808: f64, t568: f64, t836: f64, t12693: f64, t12697: f64, t12699: f64, t12701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13097 = t314 * t13096;
    let t13098 = t313 * t13097;
    let t13101 = t739 * t13096;
    let t13102 = t531 * t13101;
    let t13105 = t808 * t13096;
    let t13106 = t568 * t13105;
    let t13109 = t836 * t13096;
    let t13110 = t568 * t13109;
    let t13113 = 0.63904876589867916127e-1_f64 * t12693;
    let t13114 = 0.29792074959875355558e-1_f64 * t12697;
    let t13115 = 0.29792074959875355558e-1_f64 * t12699;
    let t13116 = 0.29792074959875355558e-1_f64 * t12701;
    (t13097, t13098, t13101, t13102, t13105, t13106, t13109, t13110, t13113, t13114, t13115, t13116)
}
