//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 804/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk804<F: Float>(t2705: F, t2709: F, t2716: F, t2726: F, t2734: F, t2737: F, t2741: F, t2753: F, t2756: F, t2759: F, t2762: F, t2815: F, t4482: F, t462: F) -> F {
    let t4484 = t462 * t4482 - t2705 + t2709 - t2716 + t2726 + t2734 - t2737 - t2741 + t2753 + t2756 + t2759 - t2762 + t2815;
    t4484
}
