//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 973/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk973<F: Float>(t16563: F, t1907: F, t5077: F, t13308: F, t19623: F, t19627: F, t5084: F, t13000: F, t5083: F, t13043: F, t19631: F, t5094: F, t12691: F, t20148: F, t5068: F, t20152: F, t5139: F) -> (F, F, F, F, F, F, F) {
    let t20293 = 2.0 / 15.0 * t5077 * t16563 * t1907;
    let t20296 = 4.0 / 15.0 * t5077 * t13308 * t19623;
    let t20299 = 2.0 / 5.0 * t5077 * t5084 * t19627;
    let t20302 = 2.0 / 3.0 * t5083 * t13000 * t19627;
    let t20305 = 8.0 / 15.0 * t13043 * t5094 * t19631;
    let t20308 = 4.0 / 15.0 * t5068 * t12691 * t20148;
    let t20311 = 2.0 / 5.0 * t5068 * t5139 * t20152;
    (t20293, t20296, t20299, t20302, t20305, t20308, t20311)
}
