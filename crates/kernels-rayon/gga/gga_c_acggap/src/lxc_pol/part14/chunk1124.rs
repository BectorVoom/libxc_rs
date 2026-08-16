//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1124/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1124(t31362: f64, t9589: f64, t4680: f64, t7337: f64, t9588: f64, t1181: f64, t26995: f64, t599: f64, t30786: f64, t30790: f64, t34866: f64, t34894: f64, t34896: f64, t34946: f64, t34958: f64, t34962: f64, t37287: f64, t37293: f64, t39551: f64, t39555: f64, t39557: f64, t39559: f64, t39563: f64) -> f64 {
    let t39567 = t31362 * t9589;
    let t39570 = t7337 * t4680 * t9588;
    let t39574 = t7337 * t1181 * t599 * t26995;
    let t39576 = -0.10718504529517434243e-3_f64 * t39551 - 0.10718504529517434243e-3_f64 * t39555 - t34866 + t37287 + t34894 + t34896 - t37293 - t39557 / 24.0_f64 - t39559 / 24.0_f64 - 0.94344276868812456204e-2_f64 * t39563 - t34946 + t34958 - t34962 - 0.10718504529517434243e-3_f64 * t30786 - 0.14291339372689912324e-3_f64 * t30790 + 0.10718504529517434243e-2_f64 * t39567 + 0.10718504529517434243e-2_f64 * t39570 + 0.10718504529517434243e-2_f64 * t39574;
    t39576
}
