//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 375/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk375(t3137: f64, t492: f64, t105: f64, t3088: f64, t3119: f64, t3124: f64, t3126: f64, t3132: f64, t3134: f64, t921: f64) -> (f64, f64, f64) {
    let t3138 = t492 * t3137;
    let t3141 = 0.28455006635676149599e-1_f64 * t105 * t3088 + 0.28455006635676149599e-1_f64 * t105 * t3119 + t3124 - 0.85365019907028448797e-1_f64 * t105 * t3126 - t3132 + 0.56910013271352299198e-1_f64 * t105 * t3134 - 0.28455006635676149599e-1_f64 * t105 * t3138;
    let t3145 = t921 * t921;
    (t3138, t3141, t3145)
}
