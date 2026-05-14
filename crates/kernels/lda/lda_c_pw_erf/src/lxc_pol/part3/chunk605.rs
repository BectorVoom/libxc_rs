//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 605/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk605<F: Float>(t1318: F, t3838: F, t1319: F, t3619: F, t571: F, t1315: F, t1446: F, t1256: F, t542: F, t1313: F, t519: F, t1278: F, t505: F, t1475: F, t219: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3840 = 16.0 / 15.0 * t1318 * t3838;
    let t3841 = t1319 * t3619;
    let t3843 = 8.0 / 15.0 * t571 * t3841;
    let t3845 = 8.0 / 15.0 * t1446 * t1315;
    let t3846 = t1256 * t542;
    let t3847 = t1313 * t3846;
    let t3849 = 4.0 / 15.0 * t519 * t3847;
    let t3850 = t505 * t1278;
    let t3851 = t1313 * t3850;
    let t3853 = 4.0 / 15.0 * t519 * t3851;
    let t3854 = t1475 * t219;
    (t3840, t3841, t3843, t3845, t3846, t3847, t3849, t3850, t3851, t3853, t3854)
}
