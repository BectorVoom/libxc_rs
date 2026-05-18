//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1183/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1183<F: Float>(t13938: F, t10027: F, t5157: F, t13777: F, t3974: F, t3976: F, t593: F, t13914: F, t13916: F, t13917: F, t13919: F, t13923: F, t13925: F, t13927: F, t13930: F, t13933: F, t13937: F) -> (F, F, F, F) {
    let t13939 = F::new(32.0) / F::new(27.0) * t13938;
    let t13941 = F::new(16.0) / F::new(15.0) * t10027 * t5157;
    let t13945 = F::new(8.0) / F::new(15.0) * t3974 * t3976 * t13777 * t593;
    let t13946 = t13914 + t13916 + F::new(0.21642082724729686) * t13917 - F::new(0.09618703433213194) * t13919 - t13923 - t13925 - t13927 - t13930 + t13933 + t13937 + t13939 - t13941 - t13945;
    (t13939, t13941, t13945, t13946)
}
