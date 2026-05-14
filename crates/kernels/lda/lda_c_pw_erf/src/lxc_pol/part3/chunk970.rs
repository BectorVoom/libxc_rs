//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 970/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk970<F: Float>(t3425: F, t4763: F, t10012: F, t1278: F, t1440: F, t4885: F, t519: F, t2146: F, t4059: F, t1325: F, t4880: F, t944: F, t3775: F, t4738: F, t1124: F, t213: F) -> (F, F, F, F, F, F, F) {
    let t12902 = 8.0 / 15.0 * t4763 * t3425;
    let t12903 = 32.0 / 45.0 * t10012;
    let t12907 = 4.0 / 5.0 * t519 * t1440 * t4885 * t1278;
    let t12908 = t2146 * t4059;
    let t12909 = 8.0 / 45.0 * t12908;
    let t12913 = 4.0 / 5.0 * t1325 * t1440 * t4880 * t944;
    let t12915 = 4.0 / 5.0 * t4738 * t3775;
    let t12916 = t1124 * t213;
    (t12902, t12903, t12907, t12909, t12913, t12915, t12916)
}
