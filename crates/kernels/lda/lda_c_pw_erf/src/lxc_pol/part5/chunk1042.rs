//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1042/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1042<F: Float>(t15685: F, t6693: F, t17657: F, t13049: F, t13052: F, t13359: F, t21694: F, t21695: F, t21696: F, t21698: F, t21700: F, t21703: F, t21706: F, t21711: F, t17664: F, t595: F, t7676: F) -> (F, F, F, F, F) {
    let t21713 = 8.0 / 5.0 * t15685 * t6693;
    let t21714 = 32.0 / 45.0 * t17657;
    let t21715 = t13049 + t13052 + t21694 + t21695 - t21696 - t13359 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714;
    let t21717 = 32.0 / 45.0 * t17664;
    let t21719 = 2.0 / 15.0 * t7676 * t595;
    (t21713, t21714, t21715, t21717, t21719)
}
