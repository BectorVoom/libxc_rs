//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 372/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk372(t169: f64, t3116: f64, t172: f64, t452: f64, t2321: f64, t894: f64, t882: f64, t203: f64, t3086: f64, t492: f64, t2334: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3117 = t3116 * t169;
    let t3118 = t3117 * t172;
    let t3119 = t452 * t3118;
    let t3122 = t894 * t2321;
    let t3124 = 0.23712505529730124666e-2_f64 * t882 * t3122;
    let t3125 = t3086 * t203;
    let t3126 = t492 * t3125;
    let t3129 = t883 * t2334;
    (t3118, t3119, t3122, t3124, t3125, t3126, t3129)
}
