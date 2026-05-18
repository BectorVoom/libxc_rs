//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1307/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1307<F: Float>(t39411: F, t49385: F, t49387: F, t56966: F, t56978: F, t56981: F, t56984: F, t57024: F, t57057: F, t57060: F, t57063: F, t24678: F, t30270: F, t39413: F, t39418: F, t49240: F, t49242: F, t49393: F, t49395: F, t56969: F, t57027: F, t57037: F, t57041: F) -> (F, F) {
    let t57312 = -F::new(0.17808333333333333333e-1) * t57057 + F::new(0.4274e0) * t57060 - F::new(0.6411e0) * t56978 + F::new(0.10685e0) * t57063 - F::new(0.94977777777777777776e-1) * t49385 + F::new(0.14246666666666666667e0) * t49387 + F::new(0.14246666666666666667e0) * t56981 - F::new(0.47488888888888888888e-1) * t56984 - F::new(0.47488888888888888888e-1) * t39411 - F::new(0.42739999999999999999e0) * t57024 + F::new(0.42739999999999999999e0) * t56966;
    let t57324 = -F::new(0.35616666666666666666e-1) * t57027 - F::new(0.11872222222222222222e0) * t56969 - F::new(0.31659259259259259258e-1) * t39413 + F::new(0.94977777777777777776e-1) * t39418 + t24678 + F::new(0.47488888888888888888e-1) * t49240 - F::new(0.14246666666666666667e0) * t49242 + F::new(0.23744444444444444444e-1) * t49393 + F::new(0.26382716049382716049e-1) * t49395 + F::new(0.73871604938271604937e-1) * t30270 + F::new(0.23744444444444444444e0) * t57037 - F::new(0.52765432098765432099e-1) * t57041;
    (t57312, t57324)
}
