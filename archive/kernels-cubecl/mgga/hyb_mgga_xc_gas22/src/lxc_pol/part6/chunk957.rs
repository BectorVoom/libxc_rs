//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 957/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk957<F: Float>(t3321: F, t6497: F, t3357: F, t6574: F, t3353: F, t809: F, t2188: F, t2289: F, t3418: F, t849: F, t8651: F, t6530: F, t6533: F, t6552: F, t8648: F, t8676: F) -> (F, F, F, F, F, F, F) {
    let t8736 = F::cast_from(4.0_f64) * t6497 * t3321;
    let t8738 = F::cast_from(0.32163958997385070134e2_f64) * t6574 * t3357;
    let t8739 = t3353 * t809;
    let t8741 = F::cast_from(4.0_f64) * t2188 * t8739;
    let t8742 = t2289 * t3418;
    let t8743 = t8742 * t849;
    let t8751 = F::cast_from(0.18541666666666666667e-1_f64) * t8651;
    let t8753 = -t6552 + F::cast_from(0.24722222222222222222e-1_f64) * t6530 - F::cast_from(0.92708333333333333333e-2_f64) * t6533 + F::cast_from(0.12361111111111111111e-1_f64) * t8676 - t8751 + F::cast_from(0.278125e-1_f64) * t8648;
    (t8736, t8738, t8739, t8741, t8743, t8751, t8753)
}
