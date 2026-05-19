//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1098/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1098<F: Float>(t10591: F, t10593: F, t10598: F, t10602: F, t10605: F, t10609: F, t10613: F, t6616: F, t6698: F, t8706: F, t8893: F, t8894: F) -> F {
    let t10741 = F::new(0.31558125e0) * t10591 + F::new(0.6311625e0) * t10593 - t6698 + F::cast_from(0.34731666666666666666e0_f64) * t6616 + F::cast_from(0.69463333333333333333e0_f64) * t8706 - t8893 - t8894 - F::new(0.20839e0) * t10598 + F::new(0.62517e0) * t10602 - F::new(0.20839e0) * t10605 + F::new(0.312585e0) * t10609 + F::new(0.312585e0) * t10613;
    t10741
}
