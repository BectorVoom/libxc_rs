//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 468/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk468<F: Float>(t443: F, t770: F, t450: F, t774: F, t133: F, t1837: F, t119: F, t132: F) -> (F, F, F, F) {
    let t1861 = t770 * t443;
    let t1864 = t774 * t450;
    let t1868 = t133 * t1837;
    let t1870 = t132 * t119;
    (t1861, t1864, t1868, t1870)
}
