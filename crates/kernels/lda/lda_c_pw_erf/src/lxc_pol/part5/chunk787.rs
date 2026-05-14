//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 787/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk787<F: Float>(t3988: F, t3992: F, t7630: F, t7663: F, t7678: F, t7682: F, t7686: F, t7690: F, t7694: F, t7697: F, t7700: F, t7704: F, t7708: F, t7712: F, t7715: F, t7718: F, t7722: F, t7726: F) -> (F,) {
    let t8040 = -t3988 + t3992 + t7630 + t7663 + t7678 - t7682 + t7686 + t7690 - t7694 - t7697 + t7700 + t7704 - t7708 - t7712 - t7715 + t7718 + t7722 - t7726;
    (t8040,)
}
