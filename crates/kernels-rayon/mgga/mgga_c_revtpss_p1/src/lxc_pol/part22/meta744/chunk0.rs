//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2812/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2812(t11007: f64, t252: f64, t786: f64, t11006: f64, t256: f64, t225: f64, t2441: f64, t39515: f64, t10504: f64, t138: f64, t886: f64, t9302: f64) -> (f64, f64, f64, f64) {
    let t41070 = t786 * t252 * t11007;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41078 = t225 * t41077;
    let t41095 = 0.11564373972601816912e-1_f64 * t39515 * t2441;
    let t41098 = t10504 * t138 * t9302 * t886;
    (t41070, t41078, t41095, t41098)
}
