//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 929/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk929<F: Float>(t5888: F, t9523: F, t5144: F, t9540: F, t5267: F, t26291: F, t29838: F, t34799: F, t34813: F, t37218: F, t37221: F, t37222: F, t37223: F, t38822: F, t38826: F, t38833: F, t38841: F, t40724: F, t42749: F, t42755: F, t44954: F, t44956: F) -> (F, F, F, F, F) {
    let t48278 = t9523 * t5888;
    let t48281 = t9540 * t5144;
    let t48284 = t9540 * t5267;
    let t48287 = t9540 * t5888;
    let t48297 = -0.71845450211182851384e0 * t40724 * t48278 - 0.71845450211182851384e0 * t26291 * t48281 + 0.95793933614910468512e0 * t29838 * t48284 + 0.71845450211182851384e0 * t34813 * t48287 - t37218 + 0.11974241701863808564e0 * t44954 - 0.17961362552795712846e0 * t44956 + t42749 - 0.20496175532535769483e-3 * t38822 + 0.12195059916630011325e-2 * t38826 - t37221 + t37222 - t37223 - 0.1440846329149835838e-2 * t34799 + 0.12195059916630011325e-2 * t38833 + t42755 - 0.17347588262831798123e-3 * t38841;
    (t48278, t48281, t48284, t48287, t48297)
}
