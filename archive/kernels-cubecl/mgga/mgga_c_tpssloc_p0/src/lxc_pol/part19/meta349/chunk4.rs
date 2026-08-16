//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1270/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1270<F: Float>(t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F) -> F {
    let t41719 = -F::cast_from(0.94977777777777777776e-1_f64) * t41678 + F::cast_from(0.47488888888888888888e-1_f64) * t41680 + F::cast_from(0.14246666666666666667e0_f64) * t41682 + F::cast_from(0.73871604938271604937e-1_f64) * t41684 + F::cast_from(0.23744444444444444444e0_f64) * t41690 - F::cast_from(0.11872222222222222222e0_f64) * t41695 - F::cast_from(0.42739999999999999999e0_f64) * t41699 - F::cast_from(0.35616666666666666666e-1_f64) * t41703 - F::cast_from(0.47488888888888888888e-1_f64) * t41707 + F::cast_from(0.4274e0_f64) * t41711 - F::cast_from(0.14246666666666666667e0_f64) * t41713 - F::cast_from(0.6411e0_f64) * t41717;
    t41719
}
