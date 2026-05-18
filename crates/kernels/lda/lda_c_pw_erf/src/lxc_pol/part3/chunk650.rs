//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 650/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk650<F: Float>(t1309: F, t3863: F, t571: F, t1401: F, t574: F, t1403: F, t559: F, t1356: F, t593: F, t1308: F, t1446: F, t1454: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3864 = t3863 * t1309;
    let t3865 = t571 * t3864;
    let t3866 = F::new(16.0) / F::new(45.0) * t3865;
    let t3867 = t574 * t1401;
    let t3868 = t559 * t1403;
    let t3869 = t3867 * t3868;
    let t3871 = F::new(8.0) / F::new(15.0) * t571 * t3869;
    let t3872 = t1356 * t593;
    let t3873 = t1308 * t3872;
    let t3875 = F::new(8.0) / F::new(15.0) * t571 * t3873;
    let t3877 = F::new(4.0) / F::new(15.0) * t1446 * t1454;
    (t3864, t3865, t3866, t3867, t3868, t3869, t3871, t3872, t3873, t3875, t3877)
}
