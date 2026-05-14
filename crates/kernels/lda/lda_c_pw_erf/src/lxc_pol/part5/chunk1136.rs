//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1136/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1136<F: Float>(t230: F, t7827: F, t10278: F, t10286: F, t13377: F, t13380: F, t21698: F, t21700: F, t21703: F, t21706: F, t21711: F, t21713: F, t21714: F, t21717: F, t21719: F, t21721: F, t21725: F, t21726: F, t21727: F, t21728: F, t21729: F, t21730: F, t21731: F, t21732: F, t21733: F, t21734: F, t21738: F) -> (F, F) {
    let t23266 = t7827 * t230;
    let t23268 = 12.0 * t13377 + t13380 + 4.0 / 3.0 * t10278 + t10286 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714 - t21717 + 4.0 / 3.0 * t23266;
    let t23269 = -t21719 - t21721 + t21725 - t21726 + t21727 - t21728 - t21729 + t21730 - t21731 + t21732 + t21733 + t21734 + t21738;
    (t23268, t23269)
}
