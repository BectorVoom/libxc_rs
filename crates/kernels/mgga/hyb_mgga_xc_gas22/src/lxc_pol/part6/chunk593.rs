//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 593/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk593<F: Float>(t2814: F, t462: F, t2341: F, t2628: F, t2737: F, t2738: F, t2741: F, t2743: F, t2745: F, t2747: F, t2753: F, t2756: F, t2759: F, t2762: F) -> (F, F) {
    let t2815 = t462 * t2814;
    let t2816 = t462 * t2747 - t2341 - t2628 + t2737 + F::cast_from(8.0_f64) * t2738 - t2741 + F::cast_from(2.0_f64) * t2743 - F::cast_from(8.0_f64) * t2745 + t2753 + t2756 - t2759 - t2762 + t2815;
    (t2815, t2816)
}
