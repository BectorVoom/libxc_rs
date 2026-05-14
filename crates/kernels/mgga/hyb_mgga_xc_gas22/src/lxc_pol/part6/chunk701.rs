//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 701/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk701<F: Float>(t2700: F, t2705: F, t2709: F, t2716: F, t2726: F, t2734: F, t2738: F, t2741: F, t2743: F, t2745: F, t2753: F, t2756: F, t2762: F, t2815: F, t3651: F, t1128: F, t17: F) -> (F, F) {
    let t3655 = -0.18311447306006545054e-3 * t2700 - t2705 + t2709 - t2716 + t2726 + t2734 - 4.0 * t2738 - t2741 + t2743 - 4.0 * t2745 + t2753 - t2756 - t2762 + t2815;
    let t3656 = t3651 + t3655;
    let t3660 = t1128 * t17;
    (t3656, t3660)
}
