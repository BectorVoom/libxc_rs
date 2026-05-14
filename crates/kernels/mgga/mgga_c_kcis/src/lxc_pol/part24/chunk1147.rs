//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1147/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1147<F: Float>(t100522: F, t100525: F, t100528: F, t100531: F, t101208: F, t14554: F, t19399: F, t26679: F, t26692: F, t27812: F, t27822: F, t27832: F, t27950: F, t27954: F, t28928: F, t7703: F, t93366: F, t95524: F) -> (F,) {
    let t101303 = -0.66327777777777777776e-2 * t100522 + 0.18534722222222222222e-2 * t7703 * t14554 * t26679 * t19399 + 0.61836467013888888889e-4 * t93366 * t28928 - 0.16581944444444444444e-2 * t100525 - 0.61890573922526041667e-5 * t27812 * t101208 + 0.66327777777777777776e-2 * t100528 + 0.11054629629629629629e-2 * t100531 + 0.61836467013888888889e-4 * t95524 * t27822 - 0.61782407407407407408e-3 * t27832 * t27950 - 0.12356481481481481482e-2 * t26692 * t28928 + 0.46336805555555555557e-3 * t27832 * t27954;
    (t101303,)
}
