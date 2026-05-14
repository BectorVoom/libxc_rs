//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1118/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1118<F: Float>(t16694: F, t16884: F, t27438: F, t52371: F, t5709: F, t7908: F, t7909: F, t94227: F, t94451: F, t94465: F, t98124: F, t98463: F, t98466: F, t98472: F, t98475: F, t98489: F, t98491: F) -> (F,) {
    let t98494 = -0.11054629629629629629e-1 * t98463 - 0.61836467013888888889e-4 * t94227 * t98466 - 0.16581944444444444444e-2 * t94451 - 0.33163888888888888888e-2 * t98472 + 0.22109259259259259258e-2 * t98475 - 0.13901041666666666667e-2 * t7908 * t5709 * t27438 * t16694 - 0.92673611111111111112e-3 * t7908 * t16884 * t7909 * t52371 - 0.92673611111111111112e-3 * t7908 * t98124 + t98489 - 0.20594135802469135802e-3 * t98491 + 0.46336805555555555556e-3 * t94465;
    (t98494,)
}
