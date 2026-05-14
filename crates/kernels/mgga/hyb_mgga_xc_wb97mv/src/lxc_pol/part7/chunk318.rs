//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 318/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk318<F: Float>(t1014: F, t1017: F, t1039: F, t1064: F, t1071: F, t1079: F, t1086: F, t221: F, t475: F, t488: F) -> (F,) {
    let t1089 = 0.53237641966666666666e-3 * t221 * t1014 * t475 + 1.0 * t1064 * t1071 - t1017 - t1039 + 0.18311447306006545054e-3 * t221 * t1014 * t488 + 0.5848223622634646207e0 * t1079 * t1086;
    (t1089,)
}
