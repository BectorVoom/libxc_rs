//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1254/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1254(t16937: f64, t28484: f64, t27369: f64, t16941: f64, t28494: f64, t7908: f64, t16694: f64, t16884: f64, t27438: f64, t52371: f64, t5709: f64, t7909: f64, t94227: f64, t94451: f64, t94465: f64, t98124: f64, t98463: f64, t98466: f64, t98472: f64, t98475: f64) -> (f64, f64) {
    let t98487 = t16937 * t28484;
    let t98489 = 0.20612155671296296296e-4_f64 * t27369 * t98487;
    let t98491 = t7908 * t16941 * t28494;
    let t98494 = -0.11054629629629629629e-1_f64 * t98463 - 0.61836467013888888889e-4_f64 * t94227 * t98466 - 0.16581944444444444444e-2_f64 * t94451 - 0.33163888888888888888e-2_f64 * t98472 + 0.22109259259259259258e-2_f64 * t98475 - 0.13901041666666666667e-2_f64 * t7908 * t5709 * t27438 * t16694 - 0.92673611111111111112e-3_f64 * t7908 * t16884 * t7909 * t52371 - 0.92673611111111111112e-3_f64 * t7908 * t98124 + t98489 - 0.20594135802469135802e-3_f64 * t98491 + 0.46336805555555555556e-3_f64 * t94465;
    (t98487, t98494)
}
