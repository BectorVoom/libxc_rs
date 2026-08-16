//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1070/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1070(t5888: f64, t9523: f64, t5144: f64, t9540: f64, t5267: f64, t26291: f64, t29838: f64, t34799: f64, t34813: f64, t37218: f64, t37221: f64, t37222: f64, t37223: f64, t38822: f64, t38826: f64, t38833: f64, t38841: f64, t40724: f64, t42749: f64, t42755: f64, t44954: f64, t44956: f64) -> (f64, f64, f64, f64, f64) {
    let t48278 = t9523 * t5888;
    let t48281 = t9540 * t5144;
    let t48284 = t9540 * t5267;
    let t48287 = t9540 * t5888;
    let t48297 = -0.71845450211182851384e0_f64 * t40724 * t48278 - 0.71845450211182851384e0_f64 * t26291 * t48281 + 0.95793933614910468512e0_f64 * t29838 * t48284 + 0.71845450211182851384e0_f64 * t34813 * t48287 - t37218 + 0.11974241701863808564e0_f64 * t44954 - 0.17961362552795712846e0_f64 * t44956 + t42749 - 0.20496175532535769483e-3_f64 * t38822 + 0.12195059916630011325e-2_f64 * t38826 - t37221 + t37222 - t37223 - 0.1440846329149835838e-2_f64 * t34799 + 0.12195059916630011325e-2_f64 * t38833 + t42755 - 0.17347588262831798123e-3_f64 * t38841;
    (t48278, t48281, t48284, t48287, t48297)
}
