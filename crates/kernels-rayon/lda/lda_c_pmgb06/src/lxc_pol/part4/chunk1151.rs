//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1151/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1151(t1186: f64, t421: f64, t7155: f64, t10670: f64, t10790: f64, t14275: f64, t14279: f64, t14283: f64, t14287: f64, t14290: f64, t14293: f64, t14297: f64, t14300: f64, t14303: f64, t14306: f64, t14308: f64, t15152: f64, t15159: f64, t15163: f64) -> f64 {
    let t15166 = t7155 * t1186 * t421;
    let t15168 = 0.1756220988170676_f64 * t10670 - 0.051799090195807085_f64 * t14275 + 0.006935985972286697_f64 * t14279 - 0.001981710277796199_f64 * t14283 - 0.003950778065781896_f64 * t14287 - 0.015803112263127583_f64 * t14290 - 0.01185233419734569_f64 * t14293 + 0.026338520438545975_f64 * t14297 + 0.03950778065781896_f64 * t14300 - 0.002972565416694299_f64 * t14303 - 0.02394846802050922_f64 * t15152 - 0.02394846802050922_f64 * t10790 + 0.3780648866776934_f64 * t14306 - 0.0002373061974330281_f64 * t14308 + 0.006584630109636494_f64 * t15159 - 0.003950778065781896_f64 * t15163 - 0.003950778065781896_f64 * t15166;
    t15168
}
