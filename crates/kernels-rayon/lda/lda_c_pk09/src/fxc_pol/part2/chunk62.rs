//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 62/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk62(t128: f64, t134: f64, t106: f64, t115: f64, t123: f64, t77: f64, t90: f64, t98: f64, t110: f64, t88: f64, t10: f64, t91: f64) -> (f64, f64, f64, f64) {
    let t135 = t128 * t134;
    let t137 = -t90 * t98 / 6.0_f64 - t106 * t98 / 6.0_f64 + t115 * t98 / 6.0_f64 - 0.10237773105191754_f64 * t77 + 1.0150830754383913_f64 + 0.14975624337724558_f64 * t123 + 0.006167148707670995_f64 * t135;
    let t141 = t110 * t88;
    let t142 = t10 * t91;
    (t135, t137, t141, t142)
}
