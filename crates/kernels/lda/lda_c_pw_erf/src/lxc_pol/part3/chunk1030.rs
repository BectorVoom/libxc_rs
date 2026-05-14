//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1030<F: Float>(t13929: F, t3899: F, t5374: F, t571: F, t1466: F, t2161: F, t3655: F, t10030: F, t5167: F, t10027: F, t5157: F, t13777: F, t3974: F, t3976: F, t593: F, t13914: F, t13916: F, t13917: F, t13919: F, t13923: F, t13925: F, t13927: F) -> (F, F, F, F, F, F, F) {
    let t13930 = 8.0 / 45.0 * t13929;
    let t13932 = t571 * t3899 * t5374;
    let t13933 = 8.0 / 15.0 * t13932;
    let t13937 = 4.0 / 15.0 * t571 * t1466 * t2161 * t3655;
    let t13938 = t10030 * t5167;
    let t13939 = 32.0 / 27.0 * t13938;
    let t13941 = 16.0 / 15.0 * t10027 * t5157;
    let t13945 = 8.0 / 15.0 * t3974 * t3976 * t13777 * t593;
    let t13946 = t13914 + t13916 + 0.21642082724729686 * t13917 - 0.09618703433213194 * t13919 - t13923 - t13925 - t13927 - t13930 + t13933 + t13937 + t13939 - t13941 - t13945;
    (t13930, t13933, t13937, t13939, t13941, t13945, t13946)
}
