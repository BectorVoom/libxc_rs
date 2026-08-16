//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1454/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454(t324: f64, t41525: f64, t41538: f64, t300: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64) -> (f64, f64, f64) {
    let t41540 = (t41525 + t41538) * t324;
    let t41542 = 0.19751673498613801407e-1_f64 * t300 * t41540;
    let t41549 = 0.18467901234567901234e0_f64 * t41306;
    let t41554 = 0.14246666666666666667e0_f64 * t41308 + 0.4274e0_f64 * t41312 - 0.6411e0_f64 * t41316 + 0.10685e0_f64 * t41320 + 0.42739999999999999999e0_f64 * t41323 - 0.35616666666666666666e-1_f64 * t41327 + t41549 - 0.47488888888888888888e-1_f64 * t41330 - 0.31659259259259259258e-1_f64 * t41332 + 0.23744444444444444444e-1_f64 * t41334 + 0.26382716049382716049e-1_f64 * t41336;
    (t41540, t41542, t41554)
}
