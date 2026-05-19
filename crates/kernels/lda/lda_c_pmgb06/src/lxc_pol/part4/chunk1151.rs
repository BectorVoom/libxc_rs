//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1151/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1151<F: Float>(t1186: F, t421: F, t7155: F, t10670: F, t10790: F, t14275: F, t14279: F, t14283: F, t14287: F, t14290: F, t14293: F, t14297: F, t14300: F, t14303: F, t14306: F, t14308: F, t15152: F, t15159: F, t15163: F) -> F {
    let t15166 = t7155 * t1186 * t421;
    let t15168 = F::cast_from(0.1756220988170676_f64) * t10670 - F::cast_from(0.051799090195807085_f64) * t14275 + F::cast_from(0.006935985972286697_f64) * t14279 - F::cast_from(0.001981710277796199_f64) * t14283 - F::cast_from(0.003950778065781896_f64) * t14287 - F::cast_from(0.015803112263127583_f64) * t14290 - F::cast_from(0.01185233419734569_f64) * t14293 + F::cast_from(0.026338520438545975_f64) * t14297 + F::cast_from(0.03950778065781896_f64) * t14300 - F::cast_from(0.002972565416694299_f64) * t14303 - F::cast_from(0.02394846802050922_f64) * t15152 - F::cast_from(0.02394846802050922_f64) * t10790 + F::cast_from(0.3780648866776934_f64) * t14306 - F::cast_from(0.0002373061974330281_f64) * t14308 + F::cast_from(0.006584630109636494_f64) * t15159 - F::cast_from(0.003950778065781896_f64) * t15163 - F::cast_from(0.003950778065781896_f64) * t15166;
    t15168
}
