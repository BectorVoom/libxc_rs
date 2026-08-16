//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 42/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk42(t60: f64, t6: f64, t123: f64, t121: f64, t21: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t124 = 0.0_f64 < t60;
    let t126 = piecewise3(t124, t60, -t60);
    let t127 = 1.0_f64 / t126;
    let t128 = t6 * t127;
    let t129 = t123 * t128;
    let t132 = 1.0_f64 + 0.53972366148531951642e-1_f64 * t121 * t129;
    let t133 = f64::ln(t132);
    let t135 = 1.0_f64 + 0.193e0_f64 * t133;
    let t136 = 1.0_f64 / t135;
    let t138 = 1.0_f64 / t21;
    let t139 = t2 * t138;
    (t126, t129, t132, t135, t136, t139)
}
