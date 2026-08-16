//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1483/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483(t378: f64, t42051: f64, t11198: f64, t340: f64, t338: f64, t3059: f64, t11119: f64, t384: f64, t225: f64, t3270: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42052 = t42051 * t378;
    let t42058 = 1.0_f64 / t11198 / t340;
    let t42059 = t338 * t42058;
    let t42060 = t42059 * t378;
    let t42061 = t3059 * t3059;
    let t42066 = 1.0_f64 / t11119 / t384;
    let t42067 = t225 * t42066;
    let t42068 = t3270 * t3270;
    let t42078 = 0.15365432098765432099e0_f64 * t41306;
    let t42083 = 0.11853333333333333334e0_f64 * t41308 + 0.35560000000000000001e0_f64 * t41312 - 0.53340000000000000002e0_f64 * t41316 + 0.88900000000000000002e-1_f64 * t41320 + 0.35560000000000000001e0_f64 * t41323 - 0.29633333333333333334e-1_f64 * t41327 + t42078 - 0.39511111111111111112e-1_f64 * t41330 - 0.26340740740740740742e-1_f64 * t41332 + 0.19755555555555555556e-1_f64 * t41334 + 0.21950617283950617284e-1_f64 * t41336;
    (t42052, t42059, t42060, t42061, t42067, t42068, t42083)
}
