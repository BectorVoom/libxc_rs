//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1052/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1052<F: Float>(t10686: F, t10688: F, t10690: F, t10694: F, t10697: F, t10699: F, t10702: F, t10704: F, t10709: F, t10712: F, t10715: F, t10718: F, t10719: F, t2171: F, t3888: F, t1325: F, t1440: F, t2166: F, t3545: F) -> (F, F, F) {
    let t14304 = 0.21642082724729686 * t10686 + 0.6492624817418906 * t10688 - 0.2885611029963958 * t10690 - t10694 + t10697 + 0.03354522822333102 * t10699 + 0.9738937226128359 * t10702 + 0.10063568466999305 * t10704 + t10709 + t10712 - t10715 + t10718 - 0.03354522822333102 * t10719;
    let t14307 = 4.0 / 45.0 * t2171 * t3888;
    let t14311 = 4.0 / 15.0 * t1325 * t1440 * t2166 * t3545;
    (t14304, t14307, t14311)
}
