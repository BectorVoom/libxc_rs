//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 526/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk526(t1165: f64, t3174: f64, t3176: f64, t1163: f64, t301: f64, t435: f64, t1160: f64, t1172: f64, t157: f64, t406: f64) -> (f64, f64, f64, f64, f64) {
    let t3178 = t1165 * t3174 * t3176;
    let t3179 = t1163 * t3178;
    let t3189 = t435 * t301;
    let t3194 = t1160 * t1172;
    let t3196 = t301 * t406 * t157;
    (t3178, t3179, t3189, t3194, t3196)
}
