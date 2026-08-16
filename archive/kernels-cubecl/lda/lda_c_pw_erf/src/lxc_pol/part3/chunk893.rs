//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 893/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk893<F: Float>(t426: F, t8945: F, t1250: F, t47: F, t1332: F, t52: F, t1568: F, t299: F, t732: F, t3257: F, t1691: F, t8924: F) -> (F, F, F, F, F, F) {
    let t8946 = t426 * t8945;
    let t8949 = F::cast_from(1.0_f64) / t47 / t1250;
    let t8962 = F::cast_from(1.0_f64) / t52 / t1332;
    let t8980 = t732 * t299 * t1568;
    let t8981 = t3257 * t8980;
    let t8983 = t1691 * t8924;
    (t8946, t8949, t8962, t8980, t8981, t8983)
}
