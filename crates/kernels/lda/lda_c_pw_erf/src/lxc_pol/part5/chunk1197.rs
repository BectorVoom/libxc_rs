//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1197/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1197<F: Float>(t529: F, t7797: F, t1325: F, t1440: F, t542: F, t15685: F, t6693: F, t17657: F, t13049: F, t13052: F, t13359: F, t21694: F, t21695: F, t21696: F, t21698: F, t21700: F, t21703: F, t21706: F) -> (F, F, F, F) {
    let t21707 = t529 * t7797;
    let t21711 = F::new(4.0) / F::new(15.0) * t1325 * t1440 * t21707 * t542;
    let t21713 = F::new(8.0) / F::new(5.0) * t15685 * t6693;
    let t21714 = F::new(32.0) / F::new(45.0) * t17657;
    let t21715 = t13049 + t13052 + t21694 + t21695 - t21696 - t13359 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714;
    (t21711, t21713, t21714, t21715)
}
