//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 965/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk965<F: Float>(t848: F, t8709: F, t8651: F, t6530: F, t6533: F, t6616: F, t6619: F, t6622: F, t6698: F, t8648: F, t8654: F, t8656: F, t8659: F, t8661: F) -> (F, F, F) {
    let t8869 = t8709 * t848;
    let t8877 = F::cast_from(0.103295e1_f64) * t8651;
    let t8887 = F::cast_from(0.1549425e1_f64) * t8648 - t8877 - F::cast_from(0.3529725e1_f64) * t8654 - F::cast_from(0.17648625e1_f64) * t8656 + F::cast_from(0.6311625e0_f64) * t8659 + F::cast_from(0.31558125e0_f64) * t8661 + F::cast_from(0.13772666666666666667e1_f64) * t6530 - F::cast_from(0.516475e0_f64) * t6533 - t6698 + F::cast_from(0.69463333333333333333e0_f64) * t6616 - F::cast_from(0.20839e0_f64) * t6619 - F::cast_from(0.20839e0_f64) * t6622;
    (t8869, t8877, t8887)
}
